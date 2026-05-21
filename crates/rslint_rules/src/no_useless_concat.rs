use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessConcat;

impl Rule for NoUselessConcat {
    fn name(&self) -> &'static str {
        "no-useless-concat"
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
        if ctx.node_text(&op) != "+" {
            return;
        }
        let left = match node.child_by_field_name("left") {
            Some(l) => l,
            None => return,
        };
        let right = match node.child_by_field_name("right") {
            Some(r) => r,
            None => return,
        };
        if left.kind() == "string" && right.kind() == "string" {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected string concatenation of literals.",
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
        assert!(lint(Box::new(NoUselessConcat), r#"var x = "ab";"#).is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUselessConcat), r#"var x = "a" + "b";"#);
        assert_eq!(d.len(), 1);
    }
}
