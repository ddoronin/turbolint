use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoWith;

impl Rule for NoWith {
    fn name(&self) -> &'static str {
        "no-with"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "with_statement" {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected use of 'with' statement.",
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
        assert!(lint(Box::new(NoWith), "var x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoWith), "with (Math) { floor(1.6); }");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-with");
    }
}
