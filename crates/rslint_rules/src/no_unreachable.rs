use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoUnreachable;
impl Rule for NoUnreachable {
    fn name(&self) -> &'static str {
        "no-unreachable"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "statement_block" {
            return;
        }
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).collect();
        let mut found_terminal = false;
        for child in children {
            if found_terminal {
                ctx.report(
                    child.start_byte() as u32,
                    child.end_byte() as u32,
                    "Unreachable code.",
                );
                return;
            }
            if matches!(
                child.kind(),
                "return_statement" | "throw_statement" | "break_statement" | "continue_statement"
            ) {
                found_terminal = true;
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
        assert!(lint(Box::new(NoUnreachable), "function f() { return 1; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoUnreachable),
            "function f() { return 1; var x = 2; }",
        );
        assert_eq!(d.len(), 1);
    }
}
