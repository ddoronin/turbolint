use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUnsafeNegation;

impl Rule for NoUnsafeNegation {
    fn name(&self) -> &'static str {
        "no-unsafe-negation"
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
        let op_text = ctx.node_text(&op);
        if op_text != "in" && op_text != "instanceof" {
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
                        format!(
                            "Unexpected negating the left operand of '{}' operator.",
                            op_text
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
        assert!(lint(Box::new(NoUnsafeNegation), "if (!(a in b)) {}").is_empty());
        assert!(lint(Box::new(NoUnsafeNegation), "if (!(a instanceof B)) {}").is_empty());
    }
    #[test]
    fn invalid_in() {
        let d = lint(Box::new(NoUnsafeNegation), "if (!a in b) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_instanceof() {
        let d = lint(Box::new(NoUnsafeNegation), "if (!a instanceof B) {}");
        assert_eq!(d.len(), 1);
    }
}
