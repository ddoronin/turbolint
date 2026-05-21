use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoNegatedInLhs;

impl Rule for NoNegatedInLhs {
    fn name(&self) -> &'static str {
        "no-negated-in-lhs"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "binary_expression" {
            return;
        }
        let op = match node.child_by_field_name("operator") {
            Some(o) => o,
            None => return,
        };
        if ctx.node_text(&op) != "in" {
            return;
        }
        let left = match node.child_by_field_name("left") {
            Some(l) => l,
            None => return,
        };
        if left.kind() == "unary_expression" {
            if let Some(uop) = left.child_by_field_name("operator") {
                if ctx.node_text(&uop) == "!" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Negating the left operand in 'in' expressions is potentially confusing.",
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
        assert!(lint(Box::new(NoNegatedInLhs), "if (!(a in b)) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoNegatedInLhs), "if (!a in b) {}");
        assert_eq!(d.len(), 1);
    }
}
