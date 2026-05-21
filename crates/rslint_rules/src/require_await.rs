use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct RequireAwait;
impl Rule for RequireAwait {
    fn name(&self) -> &'static str {
        "require-await"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "function_declaration" | "function_expression" | "arrow_function" | "method_definition"
        ) {
            return;
        }
        let text = ctx.node_text(node);
        if !text.starts_with("async") {
            return;
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if !has_await_expr(&body) {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Async function has no 'await' expression.",
            );
        }
    }
}
fn has_await_expr(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "await_expression" {
            return true;
        }
        if matches!(
            child.kind(),
            "function_declaration" | "function_expression" | "arrow_function"
        ) {
            continue;
        }
        if has_await_expr(&child) {
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
            Box::new(RequireAwait),
            "async function f() { await bar(); }"
        )
        .is_empty());
    }
}
