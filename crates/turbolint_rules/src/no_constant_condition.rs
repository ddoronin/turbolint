use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoConstantCondition;

impl Rule for NoConstantCondition {
    fn name(&self) -> &'static str {
        "no-constant-condition"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "if_statement" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    check_condition(&cond, ctx, node);
                }
            }
            "while_statement" | "do_statement" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    check_condition(&cond, ctx, node);
                }
            }
            "ternary_expression" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    if is_constant(&cond) {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Unexpected constant condition.",
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

fn check_condition(cond: &Node, ctx: &RuleContext, report_node: &Node) {
    // Unwrap parenthesized expression
    let inner = if cond.kind() == "parenthesized_expression" {
        cond.named_child(0).unwrap_or(*cond)
    } else {
        *cond
    };
    if is_constant(&inner) {
        ctx.report(
            report_node.start_byte() as u32,
            report_node.end_byte() as u32,
            "Unexpected constant condition.",
        );
    }
}

fn is_constant(node: &Node) -> bool {
    matches!(
        node.kind(),
        "true" | "false" | "number" | "string" | "null" | "undefined"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoConstantCondition), "if (x) {}").is_empty());
    }
    #[test]
    fn invalid_true() {
        let d = lint(Box::new(NoConstantCondition), "if (true) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_number() {
        let d = lint(Box::new(NoConstantCondition), "if (1) {}");
        assert_eq!(d.len(), 1);
    }
}
