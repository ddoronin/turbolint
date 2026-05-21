use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct ConsistentReturn;
impl Rule for ConsistentReturn {
    fn name(&self) -> &'static str {
        "consistent-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "function_declaration" | "function_expression" | "method_definition"
        ) {
            return;
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if body.kind() != "statement_block" {
            return;
        }
        let mut has_value = false;
        let mut has_no_value = false;
        check_returns(&body, &mut has_value, &mut has_no_value, ctx);
        if has_value && has_no_value {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Expected a return value.",
            );
        }
    }
}
fn check_returns(node: &Node, has_value: &mut bool, has_no_value: &mut bool, _ctx: &RuleContext) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "return_statement" {
            if child.named_child_count() > 0 {
                *has_value = true;
            } else {
                *has_no_value = true;
            }
        }
        if matches!(
            child.kind(),
            "function_declaration" | "function_expression" | "arrow_function"
        ) {
            continue;
        }
        check_returns(&child, has_value, has_no_value, _ctx);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn valid() {
        assert!(lint(Box::new(ConsistentReturn), "function foo() { return 1; }").is_empty());
    }
}
