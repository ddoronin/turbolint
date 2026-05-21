use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoSelfCompare;

impl Rule for NoSelfCompare {
    fn name(&self) -> &'static str {
        "no-self-compare"
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
        if !matches!(
            op_text,
            "==" | "===" | "!=" | "!==" | ">" | "<" | ">=" | "<="
        ) {
            return;
        }
        let left = match node.child_by_field_name("left") {
            Some(n) => n,
            None => return,
        };
        let right = match node.child_by_field_name("right") {
            Some(n) => n,
            None => return,
        };
        if ctx.node_text(&left) == ctx.node_text(&right) {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Comparing to itself is potentially pointless.",
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
        assert!(lint(Box::new(NoSelfCompare), "if (a === b) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoSelfCompare), "if (a === a) {}");
        assert_eq!(d.len(), 1);
    }
}
