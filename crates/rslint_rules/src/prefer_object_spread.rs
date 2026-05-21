use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct PreferObjectSpread;
impl Rule for PreferObjectSpread {
    fn name(&self) -> &'static str {
        "prefer-object-spread"
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
                let obj = func.child_by_field_name("object");
                let prop = func.child_by_field_name("property");
                if let (Some(o), Some(p)) = (obj, prop) {
                    if ctx.node_text(&o) == "Object" && ctx.node_text(&p) == "assign" {
                        if let Some(args) = node.child_by_field_name("arguments") {
                            if let Some(first) = args.named_child(0) {
                                if first.kind() == "object" {
                                    ctx.report(
                                        node.start_byte() as u32,
                                        node.end_byte() as u32,
                                        "Use object spread instead of Object.assign().",
                                    );
                                }
                            }
                        }
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
        assert!(lint(Box::new(PreferObjectSpread), "var x = { ...obj };").is_empty());
    }
}
