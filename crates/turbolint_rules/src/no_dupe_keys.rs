use std::collections::HashMap;
use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoDupeKeys;

#[derive(Default)]
struct KeyInfo {
    has_get: bool,
    has_set: bool,
    has_value: bool,
}

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
        let mut keys: HashMap<String, KeyInfo> = HashMap::new();
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            let (key_node, kind) = match child.kind() {
                "pair" => {
                    if let Some(key) = child.child_by_field_name("key") {
                        (key, PropertyKind::Value)
                    } else {
                        continue;
                    }
                }
                "method_definition" => {
                    if let Some(name) = child.child_by_field_name("name") {
                        let source = ctx.source_text();
                        let before_name = source[child.start_byte()..name.start_byte()].trim();
                        let kind = if before_name == "get" {
                            PropertyKind::Get
                        } else if before_name == "set" {
                            PropertyKind::Set
                        } else {
                            PropertyKind::Value
                        };
                        (name, kind)
                    } else {
                        continue;
                    }
                }
                "shorthand_property_identifier" => {
                    let key_text = ctx.node_text(&child);
                    let entry = keys.entry(key_text.to_string()).or_default();
                    if entry.has_value || entry.has_get || entry.has_set {
                        ctx.report(
                            child.start_byte() as u32,
                            child.end_byte() as u32,
                            format!("Duplicate key '{}'.", key_text),
                        );
                    } else {
                        entry.has_value = true;
                    }
                    continue;
                }
                _ => continue,
            };

            let key_text = ctx.node_text(&key_node);
            let entry = keys.entry(key_text.to_string()).or_default();

            let is_dupe = match kind {
                PropertyKind::Get => {
                    if entry.has_get || entry.has_value {
                        true
                    } else {
                        entry.has_get = true;
                        false
                    }
                }
                PropertyKind::Set => {
                    if entry.has_set || entry.has_value {
                        true
                    } else {
                        entry.has_set = true;
                        false
                    }
                }
                PropertyKind::Value => {
                    if entry.has_value || entry.has_get || entry.has_set {
                        true
                    } else {
                        entry.has_value = true;
                        false
                    }
                }
            };

            if is_dupe {
                ctx.report(
                    key_node.start_byte() as u32,
                    key_node.end_byte() as u32,
                    format!("Duplicate key '{}'.", key_text),
                );
            }
        }
    }
}

enum PropertyKind {
    Value,
    Get,
    Set,
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
    fn valid_getter_setter() {
        assert!(lint(
            Box::new(NoDupeKeys),
            "var obj = { get x() {}, set x(v) {} };"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoDupeKeys), "var obj = { a: 1, a: 2 };");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_dupe_getter() {
        let d = lint(
            Box::new(NoDupeKeys),
            "var obj = { get x() {}, get x() {} };",
        );
        assert_eq!(d.len(), 1);
    }
}
