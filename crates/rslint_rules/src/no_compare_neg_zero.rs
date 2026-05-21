use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoCompareNegZero;

impl Rule for NoCompareNegZero {
    fn name(&self) -> &'static str {
        "no-compare-neg-zero"
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
        let left = node.child_by_field_name("left");
        let right = node.child_by_field_name("right");
        if left.is_some_and(|n| is_neg_zero(&n, ctx)) || right.is_some_and(|n| is_neg_zero(&n, ctx))
        {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "Do not use the '{}' operator to compare against -0.",
                    ctx.node_text(&op)
                ),
            );
        }
    }
}

fn is_neg_zero(node: &Node, ctx: &RuleContext) -> bool {
    if node.kind() != "unary_expression" {
        return false;
    }
    let op = match node.child_by_field_name("operator") {
        Some(o) => o,
        None => return false,
    };
    if ctx.node_text(&op) != "-" {
        return false;
    }
    let arg = match node.child_by_field_name("argument") {
        Some(a) => a,
        None => return false,
    };
    arg.kind() == "number" && ctx.node_text(&arg) == "0"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoCompareNegZero), "if (x === 0) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoCompareNegZero), "if (x === -0) {}");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-compare-neg-zero");
    }
}
