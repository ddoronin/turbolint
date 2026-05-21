use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoContinue;

impl Rule for NoContinue {
    fn name(&self) -> &'static str {
        "no-continue"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "continue_statement" {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected use of continue statement.",
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
        assert!(lint(
            Box::new(NoContinue),
            "for (var i = 0; i < 10; i++) { break; }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoContinue),
            "for (var i = 0; i < 10; i++) { continue; }",
        );
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-continue");
    }
}
