use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct PreferObjectHasOwn;
impl Rule for PreferObjectHasOwn {
    fn name(&self) -> &'static str {
        "prefer-object-has-own"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        let text = ctx.node_text(node);
        if text.contains("Object.prototype.hasOwnProperty.call")
            || text.contains("Object.hasOwnProperty.call")
        {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Use Object.hasOwn() instead.",
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
        assert!(lint(Box::new(PreferObjectHasOwn), "Object.hasOwn(obj, 'foo');").is_empty());
    }
}
