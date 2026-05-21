use rslint_core::ast_helpers::has_child_of_kind;
use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct GroupedAccessorPairs;
impl Rule for GroupedAccessorPairs {
    fn name(&self) -> &'static str {
        "grouped-accessor-pairs"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "class_body" && node.kind() != "object" {
            return;
        }
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).collect();
        for (i, child) in children.iter().enumerate() {
            if child.kind() != "method_definition" && child.kind() != "pair" {
                continue;
            }
            let is_getter = has_child_of_kind(child, "get");
            let is_setter = has_child_of_kind(child, "set");
            if !is_getter && !is_setter {
                continue;
            }
            let name = match child.child_by_field_name("name") {
                Some(n) => ctx.node_text(&n).to_string(),
                None => continue,
            };
            let looking_for = if is_getter { "set" } else { "get" };
            for (j, other) in children.iter().enumerate() {
                if j == i {
                    continue;
                }
                if !has_child_of_kind(other, looking_for) {
                    continue;
                }
                if let Some(on) = other.child_by_field_name("name") {
                    if ctx.node_text(&on) == name && (j as isize - i as isize).abs() > 1 {
                        ctx.report(
                            child.start_byte() as u32,
                            child.end_byte() as u32,
                            format!("Accessor pair for '{}' should be grouped.", name),
                        );
                        break;
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
        assert!(lint(
            Box::new(GroupedAccessorPairs),
            "var obj = { get a() { return 1; }, set a(v) {} };"
        )
        .is_empty());
    }
}
