use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoProto;

impl Rule for NoProto {
    fn name(&self) -> &'static str {
        "no-proto"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "member_expression" {
            if let Some(prop) = node.child_by_field_name("property") {
                if ctx.node_text(&prop) == "__proto__" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "The '__proto__' property is deprecated.",
                    );
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
        assert!(lint(Box::new(NoProto), "Object.getPrototypeOf(obj);").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoProto), "var x = obj.__proto__;");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-proto");
    }
}
