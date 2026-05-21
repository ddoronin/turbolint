use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoNegatedCondition;

impl Rule for NoNegatedCondition {
    fn name(&self) -> &'static str {
        "no-negated-condition"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "if_statement" {
            return;
        }
        // Only matters if there's an else clause
        if node.child_by_field_name("alternative").is_none() {
            return;
        }
        let cond = match node.child_by_field_name("condition") {
            Some(c) => c,
            None => return,
        };
        // Unwrap parens
        let inner = if cond.kind() == "parenthesized_expression" {
            cond.named_child(0).unwrap_or(cond)
        } else {
            cond
        };
        if inner.kind() == "unary_expression" {
            if let Some(op) = inner.child_by_field_name("operator") {
                if ctx.node_text(&op) == "!" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Unexpected negated condition.",
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
        assert!(lint(Box::new(NoNegatedCondition), "if (!a) {}").is_empty());
        assert!(lint(Box::new(NoNegatedCondition), "if (a) {} else {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoNegatedCondition), "if (!a) {} else {}");
        assert_eq!(d.len(), 1);
    }
}
