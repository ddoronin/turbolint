use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoEqNull;

impl Rule for NoEqNull {
    fn name(&self) -> &'static str {
        "no-eq-null"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
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
        if op_text != "==" && op_text != "!=" {
            return;
        }
        let left = node.child_by_field_name("left");
        let right = node.child_by_field_name("right");
        let has_null = left.is_some_and(|n| ctx.node_text(&n) == "null")
            || right.is_some_and(|n| ctx.node_text(&n) == "null");
        if has_null {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "Use '{}==' to compare with null.",
                    &op_text[..op_text.len() - 1]
                ),
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
        assert!(lint(Box::new(NoEqNull), "if (x === null) {}").is_empty());
        assert!(lint(Box::new(NoEqNull), "if (x !== null) {}").is_empty());
    }
    #[test]
    fn invalid_eq() {
        let d = lint(Box::new(NoEqNull), "if (x == null) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_neq() {
        let d = lint(Box::new(NoEqNull), "if (x != null) {}");
        assert_eq!(d.len(), 1);
    }
}
