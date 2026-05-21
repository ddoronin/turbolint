use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoLabels;

impl Rule for NoLabels {
    fn name(&self) -> &'static str {
        "no-labels"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "labeled_statement" {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected labeled statement.",
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
        assert!(lint(Box::new(NoLabels), "for (;;) { break; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoLabels), "label: for (;;) { break label; }");
        assert_eq!(d.len(), 1);
    }
}
