use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoReturnAwait;
impl Rule for NoReturnAwait {
    fn name(&self) -> &'static str {
        "no-return-await"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "return_statement" {
            return;
        }
        if let Some(arg) = node.named_child(0) {
            if arg.kind() == "await_expression" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Redundant use of `await` on a return value.",
                );
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
        assert!(lint(
            Box::new(NoReturnAwait),
            "async function f() { return foo(); }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoReturnAwait),
            "async function f() { return await foo(); }",
        );
        assert_eq!(d.len(), 1);
    }
}
