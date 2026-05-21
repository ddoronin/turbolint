use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessReturn;

impl Rule for NoUselessReturn {
    fn name(&self) -> &'static str {
        "no-useless-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "return_statement" || node.named_child_count() > 0 {
            return;
        }
        // Check if this is the last statement in a function body
        if let Some(parent) = node.parent() {
            if parent.kind() == "statement_block" {
                if let Some(gp) = parent.parent() {
                    if matches!(
                        gp.kind(),
                        "function_declaration"
                            | "function_expression"
                            | "arrow_function"
                            | "method_definition"
                    ) {
                        // Check if it's the last statement
                        let last = parent.named_child(parent.named_child_count().saturating_sub(1));
                        if let Some(last_stmt) = last {
                            if last_stmt.id() == node.id() {
                                ctx.report(
                                    node.start_byte() as u32,
                                    node.end_byte() as u32,
                                    "Unnecessary return statement.",
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUselessReturn), "function foo() { return 1; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoUselessReturn),
            "function foo() { bar(); return; }",
        );
        assert_eq!(d.len(), 1);
    }
}
