use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct UseIsnan;
impl Rule for UseIsnan {
    fn name(&self) -> &'static str {
        "use-isnan"
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
        if !matches!(
            op_text,
            "==" | "===" | "!=" | "!==" | "<" | ">" | "<=" | ">="
        ) {
            return;
        }
        let left = node.child_by_field_name("left");
        let right = node.child_by_field_name("right");
        if left.is_some_and(|n| ctx.node_text(&n) == "NaN")
            || right.is_some_and(|n| ctx.node_text(&n) == "NaN")
        {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Use Number.isNaN() instead of comparison with NaN.",
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
        assert!(lint(Box::new(UseIsnan), "Number.isNaN(x);").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(UseIsnan), "if (x === NaN) {}");
        assert_eq!(d.len(), 1);
    }
}
