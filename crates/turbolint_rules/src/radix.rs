use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct Radix;
impl Rule for Radix {
    fn name(&self) -> &'static str {
        "radix"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        if let Some(func) = node.child_by_field_name("function") {
            if ctx.node_text(&func) == "parseInt" {
                if let Some(args) = node.child_by_field_name("arguments") {
                    if args.named_child_count() < 2 {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Missing radix parameter.",
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
        assert!(lint(Box::new(Radix), "parseInt('10', 10);").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(Radix), "parseInt('10');");
        assert_eq!(d.len(), 1);
    }
}
