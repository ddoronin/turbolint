use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoImplicitCoercion;
impl Rule for NoImplicitCoercion {
    fn name(&self) -> &'static str {
        "no-implicit-coercion"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "unary_expression" {
            return;
        }
        if let Some(op) = node.child_by_field_name("operator") {
            let op_text = ctx.node_text(&op);
            if op_text == "+" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Use Number() instead of unary + for type coercion.",
                );
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
        assert!(lint(Box::new(NoImplicitCoercion), "var x = Number(y);").is_empty());
    }
}
