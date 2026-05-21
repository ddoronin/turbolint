use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct FuncNameMatching;
impl Rule for FuncNameMatching {
    fn name(&self) -> &'static str {
        "func-name-matching"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "variable_declarator" {
            return;
        }
        let name = match node.child_by_field_name("name") {
            Some(n) if n.kind() == "identifier" => ctx.node_text(&n),
            _ => return,
        };
        let value = match node.child_by_field_name("value") {
            Some(v) => v,
            None => return,
        };
        if value.kind() == "function_expression" {
            if let Some(fn_name) = value.child_by_field_name("name") {
                if ctx.node_text(&fn_name) != name {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        format!(
                            "Function name '{}' does not match variable name '{}'.",
                            ctx.node_text(&fn_name),
                            name
                        ),
                    );
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
        assert!(lint(Box::new(FuncNameMatching), "var foo = function foo() {};").is_empty());
    }
}
