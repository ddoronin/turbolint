use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoDupeClassMembers;

impl Rule for NoDupeClassMembers {
    fn name(&self) -> &'static str {
        "no-dupe-class-members"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "class_body" {
            return;
        }
        let mut members: Vec<(String, bool)> = Vec::new(); // (name, is_static)
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() != "method_definition" {
                continue;
            }
            let name = match child.child_by_field_name("name") {
                Some(n) => ctx.node_text(&n).to_string(),
                None => continue,
            };
            // Check if static
            let mut is_static = false;
            let mut inner_cursor = child.walk();
            for c in child.children(&mut inner_cursor) {
                if c.kind() == "static" {
                    is_static = true;
                    break;
                }
            }
            let key = (name.clone(), is_static);
            if members.contains(&key) {
                ctx.report(
                    child.start_byte() as u32,
                    child.end_byte() as u32,
                    format!("Duplicate name '{}'.", name),
                );
            } else {
                members.push(key);
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
            Box::new(NoDupeClassMembers),
            "class Foo { bar() {} baz() {} }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoDupeClassMembers),
            "class Foo { bar() {} bar() {} }",
        );
        assert_eq!(d.len(), 1);
    }
}
