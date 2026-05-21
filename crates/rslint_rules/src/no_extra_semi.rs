use rslint_core::context::RuleContext;
use rslint_core::diagnostic::{Fix, Severity, Span};
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
            let start = node.start_byte() as u32;
            let end = node.end_byte() as u32;
            ctx.report_with_fix(
                start,
                end,
                "Unnecessary semicolon.",
                Fix {
                    range: Span { start, end },
                    text: String::new(),
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{assert_fix, lint};

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoExtraSemi), "var x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoExtraSemi), "var x = 1;;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn fix_removes_extra_semi() {
        assert_fix(Box::new(NoExtraSemi), "var x = 1;;", "var x = 1;");
    }
    #[test]
    fn fix_multiple() {
        assert_fix(Box::new(NoExtraSemi), "var x = 1;;;", "var x = 1;");
    }
}
