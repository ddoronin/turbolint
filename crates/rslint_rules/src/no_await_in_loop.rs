use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoAwaitInLoop;

impl Rule for NoAwaitInLoop {
    fn name(&self) -> &'static str {
        "no-await-in-loop"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "await_expression" {
            return;
        }
        let mut current = node.parent();
        while let Some(p) = current {
            if matches!(
                p.kind(),
                "for_statement" | "for_in_statement" | "while_statement" | "do_statement"
            ) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Unexpected `await` inside a loop.",
                );
                return;
            }
            // Stop at function boundaries
            if matches!(
                p.kind(),
                "function_declaration"
                    | "function_expression"
                    | "arrow_function"
                    | "generator_function"
                    | "generator_function_declaration"
                    | "method_definition"
            ) {
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
            Box::new(NoAwaitInLoop),
            "async function foo() { await bar(); }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoAwaitInLoop),
            "async function foo() { for (var i = 0; i < 10; i++) { await bar(); } }",
        );
        assert_eq!(d.len(), 1);
    }
}
