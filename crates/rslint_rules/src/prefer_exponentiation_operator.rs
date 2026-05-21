use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct PreferExponentiationOperator;
impl Rule for PreferExponentiationOperator {
    fn name(&self) -> &'static str {
        "prefer-exponentiation-operator"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        if let Some(func) = node.child_by_field_name("function") {
            if func.kind() == "member_expression" {
                let obj = func.child_by_field_name("object");
                let prop = func.child_by_field_name("property");
                if let (Some(o), Some(p)) = (obj, prop) {
                    if ctx.node_text(&o) == "Math" && ctx.node_text(&p) == "pow" {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Use the '**' operator instead of 'Math.pow()'.",
                        );
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
        assert!(lint(Box::new(PreferExponentiationOperator), "var x = 2 ** 3;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(PreferExponentiationOperator),
            "var x = Math.pow(2, 3);",
        );
        assert_eq!(d.len(), 1);
    }
}
