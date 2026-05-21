use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoEmptyFunction;

impl Rule for NoEmptyFunction {
    fn name(&self) -> &'static str {
        "no-empty-function"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "function_declaration"
                | "function_expression"
                | "arrow_function"
                | "method_definition"
                | "generator_function"
                | "generator_function_declaration"
        ) {
            return;
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if body.kind() == "statement_block" && body.named_child_count() == 0 {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected empty function.",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoEmptyFunction), "function foo() { bar(); }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoEmptyFunction), "function foo() {}");
        assert_eq!(d.len(), 1);
    }
}
