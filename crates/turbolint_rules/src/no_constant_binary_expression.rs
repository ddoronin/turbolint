use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoConstantBinaryExpression;

impl Rule for NoConstantBinaryExpression {
    fn name(&self) -> &'static str {
        "no-constant-binary-expression"
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
        let left = match node.child_by_field_name("left") {
            Some(l) => l,
            None => return,
        };
        let right = match node.child_by_field_name("right") {
            Some(r) => r,
            None => return,
        };

        // new X === Y or {} === Y
        if matches!(op_text, "===" | "==" | "!==" | "!=")
            && (is_always_new(&left) || is_always_new(&right))
        {
            ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Unexpected constant binary expression. Comparisons with newly constructed objects are always falsy.",
                );
        }

        // null ?? X, "str" || X
        if matches!(op_text, "||" | "??") && is_constant_truthy(&left) {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected constant binary expression. The left side is always truthy.",
            );
        }
    }
}

fn is_always_new(node: &Node) -> bool {
    matches!(node.kind(), "new_expression" | "object" | "array" | "class")
}

fn is_constant_truthy(node: &Node) -> bool {
    matches!(
        node.kind(),
        "string" | "template_string" | "object" | "array" | "class" | "true"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoConstantBinaryExpression), "if (x === y) {}").is_empty());
    }
    #[test]
    fn invalid_new() {
        let d = lint(
            Box::new(NoConstantBinaryExpression),
            "if (new Foo() === bar) {}",
        );
        assert_eq!(d.len(), 1);
    }
}
