use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoSequences;

impl Rule for NoSequences {
    fn name(&self) -> &'static str {
        "no-sequences"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "sequence_expression" {
            return;
        }
        // Allow in for-loop init/update
        if let Some(parent) = node.parent() {
            if parent.kind() == "for_statement" {
                return;
            }
        }
        ctx.report(
            node.start_byte() as u32,
            node.end_byte() as u32,
            "Unexpected use of comma operator.",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoSequences), "var x = 1;").is_empty());
        assert!(lint(Box::new(NoSequences), "for (a = 0, b = 0;;) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoSequences), "var x = (1, 2);");
        assert_eq!(d.len(), 1);
    }
}
