use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct GetterReturn;

impl Rule for GetterReturn {
    fn name(&self) -> &'static str {
        "getter-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "method_definition" {
            return;
        }
        // Check for `get` keyword
        let mut cursor = node.walk();
        let mut is_getter = false;
        for child in node.children(&mut cursor) {
            if child.kind() == "get" {
                is_getter = true;
                break;
            }
        }
        if !is_getter {
            return;
        }
        // Check if body contains a return with a value
        if let Some(body) = node.child_by_field_name("body") {
            if !has_return_value(&body) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Expected to return a value in getter.",
                );
            }
        }
    }
}

fn has_return_value(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "return_statement" && child.named_child_count() > 0 {
            return true;
        }
        if matches!(
            child.kind(),
            "function_declaration"
                | "function_expression"
                | "arrow_function"
                | "generator_function"
                | "generator_function_declaration"
        ) {
            continue;
        }
        if has_return_value(&child) {
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
            Box::new(GetterReturn),
            "var obj = { get foo() { return 1; } };"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(GetterReturn), "var obj = { get foo() { } };");
        assert_eq!(d.len(), 1);
    }
}
