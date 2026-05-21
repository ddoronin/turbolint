use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct ClassMethodsUseThis;
impl Rule for ClassMethodsUseThis {
    fn name(&self) -> &'static str {
        "class-methods-use-this"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "method_definition" {
            return;
        }
        // Skip static, get, set, constructor
        let mut cursor = node.walk();
        for c in node.children(&mut cursor) {
            if matches!(c.kind(), "static" | "get" | "set") {
                return;
            }
        }
        if let Some(name) = node.child_by_field_name("name") {
            if ctx.node_text(&name) == "constructor" {
                return;
            }
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if !has_this(&body) {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Expected 'this' to be used by class method.",
            );
        }
    }
}
fn has_this(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "this" {
            return true;
        }
        if matches!(
            child.kind(),
            "function_declaration" | "function_expression" | "arrow_function" | "class_declaration"
        ) {
            continue;
        }
        if has_this(&child) {
            return true;
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn valid() {
        assert!(lint(
            Box::new(ClassMethodsUseThis),
            "class Foo { bar() { this.x; } }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(ClassMethodsUseThis),
            "class Foo { bar() { return 1; } }",
        );
        assert_eq!(d.len(), 1);
    }
}
