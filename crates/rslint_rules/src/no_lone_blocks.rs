use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoLoneBlocks;
impl Rule for NoLoneBlocks {
    fn name(&self) -> &'static str {
        "no-lone-blocks"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "statement_block" {
            return;
        }
        if let Some(parent) = node.parent() {
            // Lone block: parent is another statement_block or program
            if matches!(parent.kind(), "statement_block" | "program") {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Block is redundant.",
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
        assert!(lint(Box::new(NoLoneBlocks), "if (x) { foo(); }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoLoneBlocks), "{ foo(); }");
        assert_eq!(d.len(), 1);
    }
}
