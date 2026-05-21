use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct ArrayCallbackReturn;
const ARRAY_METHODS: &[&str] = &[
    "map",
    "filter",
    "find",
    "findIndex",
    "every",
    "some",
    "reduce",
    "reduceRight",
    "sort",
    "flatMap",
];
impl Rule for ArrayCallbackReturn {
    fn name(&self) -> &'static str {
        "array-callback-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(node.kind(), "function_expression" | "arrow_function") {
            return;
        }
        if let Some(parent) = node.parent() {
            if parent.kind() != "arguments" {
                return;
            }
            if let Some(call) = parent.parent() {
                if call.kind() != "call_expression" {
                    return;
                }
                if let Some(func) = call.child_by_field_name("function") {
                    if func.kind() == "member_expression" {
                        if let Some(prop) = func.child_by_field_name("property") {
                            if ARRAY_METHODS.contains(&ctx.node_text(&prop)) {
                                if let Some(body) = node.child_by_field_name("body") {
                                    if body.kind() == "statement_block" && !has_return(&body) {
                                        ctx.report(
                                            node.start_byte() as u32,
                                            node.end_byte() as u32,
                                            "Expected to return a value in this callback.",
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
}
fn has_return(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "return_statement" && child.named_child_count() > 0 {
            return true;
        }
        if matches!(
            child.kind(),
            "function_declaration" | "function_expression" | "arrow_function"
        ) {
            continue;
        }
        if has_return(&child) {
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
            Box::new(ArrayCallbackReturn),
            "[1,2].map(function(x) { return x * 2; });"
        )
        .is_empty());
    }
}
