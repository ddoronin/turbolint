use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoPromiseExecutorReturn;

impl Rule for NoPromiseExecutorReturn {
    fn name(&self) -> &'static str {
        "no-promise-executor-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "return_statement" || node.named_child_count() == 0 {
            return;
        }
        // Walk up to find if we're in a Promise executor
        let mut current = node.parent();
        while let Some(p) = current {
            if matches!(
                p.kind(),
                "function_expression" | "arrow_function" | "function_declaration"
            ) {
                // Check if parent is arguments of new Promise()
                if let Some(args) = p.parent() {
                    if args.kind() == "arguments" {
                        if let Some(new_expr) = args.parent() {
                            if new_expr.kind() == "new_expression" {
                                if let Some(constructor) =
                                    new_expr.child_by_field_name("constructor")
                                {
                                    if ctx.node_text(&constructor) == "Promise" {
                                        ctx.report(
                                            node.start_byte() as u32,
                                            node.end_byte() as u32,
                                            "Return statement is not allowed in Promise executor.",
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                return;
            }
            current = p.parent();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(
            Box::new(NoPromiseExecutorReturn),
            "new Promise(function(resolve) { resolve(1); });"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoPromiseExecutorReturn),
            "new Promise(function(resolve) { return 1; });",
        );
        assert_eq!(d.len(), 1);
    }
}
