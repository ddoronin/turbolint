use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct PreferSpread;
impl Rule for PreferSpread {
    fn name(&self) -> &'static str {
        "prefer-spread"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        if let Some(func) = node.child_by_field_name("function") {
            if func.kind() == "member_expression" {
                if let Some(prop) = func.child_by_field_name("property") {
                    if ctx.node_text(&prop) == "apply" {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Use the spread operator instead of .apply().",
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
        assert!(lint(Box::new(PreferSpread), "foo(...args);").is_empty());
    }
}
