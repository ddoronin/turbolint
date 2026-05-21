use rslint_core::ast_helpers::has_child_of_kind;
use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct AccessorPairs;
impl Rule for AccessorPairs {
    fn name(&self) -> &'static str {
        "accessor-pairs"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "object" && node.kind() != "class_body" {
            return;
        }
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).collect();
        let mut setters: Vec<String> = Vec::new();
        let mut getters: Vec<String> = Vec::new();
        for child in &children {
            if has_child_of_kind(child, "set") {
                if let Some(n) = child.child_by_field_name("name") {
                    setters.push(ctx.node_text(&n).to_string());
                }
            }
            if has_child_of_kind(child, "get") {
                if let Some(n) = child.child_by_field_name("name") {
                    getters.push(ctx.node_text(&n).to_string());
                }
            }
        }
        for s in &setters {
            if !getters.contains(s) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!("Setter '{}' is not accompanied by a getter.", s),
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
            Box::new(AccessorPairs),
            "var obj = { get a() {}, set a(v) {} };"
        )
        .is_empty());
    }
}
