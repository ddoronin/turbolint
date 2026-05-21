use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoUnusedExpressions;
impl Rule for NoUnusedExpressions {
    fn name(&self) -> &'static str {
        "no-unused-expressions"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "expression_statement" {
            return;
        }
        if let Some(expr) = node.named_child(0) {
            match expr.kind() {
                "call_expression"
                | "new_expression"
                | "assignment_expression"
                | "augmented_assignment_expression"
                | "update_expression"
                | "yield_expression"
                | "await_expression"
                | "unary_expression" => return,
                "string" => return,          // directive like "use strict"
                "template_string" => return, // tagged templates
                _ => {}
            }
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Expected an assignment or function call and instead saw an expression.",
            );
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUnusedExpressions), "foo();").is_empty());
    }
}
