use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoTernary;

impl Rule for NoTernary {
    fn name(&self) -> &'static str {
        "no-ternary"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "ternary_expression" {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Ternary operator used.",
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
        assert!(lint(Box::new(NoTernary), "if (x) { a = 1; } else { a = 2; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoTernary), "var foo = isBar ? baz : qux;");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-ternary");
    }
}
