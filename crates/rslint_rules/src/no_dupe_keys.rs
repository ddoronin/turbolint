use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoDupeKeys;

impl Rule for NoDupeKeys {
    fn name(&self) -> &'static str {
        "no-dupe-keys"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "object" {
            return;
        }
        let mut keys: Vec<String> = Vec::new();
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "pair" {
                if let Some(key) = child.child_by_field_name("key") {
                    let key_text = ctx.node_text(&key).to_string();
                    if keys.contains(&key_text) {
                        ctx.report(
                            key.start_byte() as u32,
                            key.end_byte() as u32,
                            format!("Duplicate key '{}'.", key_text),
                        );
                    } else {
                        keys.push(key_text);
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
        assert!(lint(Box::new(NoDupeKeys), "var obj = { a: 1, b: 2 };").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoDupeKeys), "var obj = { a: 1, a: 2 };");
        assert_eq!(d.len(), 1);
    }
}
