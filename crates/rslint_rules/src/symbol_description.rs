use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct SymbolDescription;
impl Rule for SymbolDescription {
    fn name(&self) -> &'static str {
        "symbol-description"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        if let Some(func) = node.child_by_field_name("function") {
            if ctx.node_text(&func) == "Symbol" {
                if let Some(args) = node.child_by_field_name("arguments") {
                    if args.named_child_count() == 0 {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Expected Symbol to have a description.",
                        );
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
        assert!(lint(Box::new(SymbolDescription), "Symbol('desc');").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(SymbolDescription), "Symbol();");
        assert_eq!(d.len(), 1);
    }
}
