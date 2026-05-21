use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct PreferRestParams;
impl Rule for PreferRestParams {
    fn name(&self) -> &'static str {
        "prefer-rest-params"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "identifier" && ctx.node_text(node) == "arguments" {
            // Skip if it's a property access target (obj.arguments)
            if let Some(parent) = node.parent() {
                if parent.kind() == "member_expression" {
                    if let Some(prop) = parent.child_by_field_name("property") {
                        if prop.id() == node.id() {
                            return;
                        }
                    }
                }
            }
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Use the rest parameters instead of 'arguments'.",
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
        assert!(lint(
            Box::new(PreferRestParams),
            "function foo(...args) { return args; }"
        )
        .is_empty());
    }
}
