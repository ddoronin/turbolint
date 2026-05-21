use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUnneededTernary;

impl Rule for NoUnneededTernary {
    fn name(&self) -> &'static str {
        "no-unneeded-ternary"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "ternary_expression" {
            return;
        }
        let consequence = match node.child_by_field_name("consequence") {
            Some(c) => c,
            None => return,
        };
        let alternative = match node.child_by_field_name("alternative") {
            Some(a) => a,
            None => return,
        };
        let cons_text = ctx.node_text(&consequence);
        let alt_text = ctx.node_text(&alternative);
        if (cons_text == "true" && alt_text == "false")
            || (cons_text == "false" && alt_text == "true")
        {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unnecessary use of boolean literals in conditional expression.",
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
        assert!(lint(Box::new(NoUnneededTernary), "var x = a ? b : c;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUnneededTernary), "var x = a ? true : false;");
        assert_eq!(d.len(), 1);
    }
}
