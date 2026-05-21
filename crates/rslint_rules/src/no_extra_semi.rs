use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoExtraSemi;

impl Rule for NoExtraSemi {
    fn name(&self) -> &'static str {
        "no-extra-semi"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "empty_statement" {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unnecessary semicolon.",
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
        assert!(lint(Box::new(NoExtraSemi), "var x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoExtraSemi), "var x = 1;;");
        assert_eq!(d.len(), 1);
    }
}
