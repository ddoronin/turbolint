use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoNestedTernary;

impl Rule for NoNestedTernary {
    fn name(&self) -> &'static str {
        "no-nested-ternary"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "ternary_expression" {
            if let Some(parent) = node.parent() {
                if parent.kind() == "ternary_expression" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Do not nest ternary expressions.",
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
        assert!(lint(Box::new(NoNestedTernary), "var foo = isBar ? baz : qux;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoNestedTernary),
            "var foo = bar ? baz : qux ? quxx : foobar;",
        );
        assert_eq!(d.len(), 1);
    }
}
