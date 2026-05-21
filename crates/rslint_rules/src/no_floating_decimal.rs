use rslint_core::context::RuleContext;
use rslint_core::diagnostic::{Fix, Severity, Span};
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoFloatingDecimal;

impl Rule for NoFloatingDecimal {
    fn name(&self) -> &'static str {
        "no-floating-decimal"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "number" {
            return;
        }
        let text = ctx.node_text(node);
        let start = node.start_byte() as u32;
        let end = node.end_byte() as u32;
        if text.starts_with('.') {
            ctx.report_with_fix(
                start,
                end,
                "A leading decimal point can be confused with a dot.",
                Fix {
                    range: Span { start, end },
                    text: format!("0{text}"),
                },
            );
        } else if text.ends_with('.') {
            ctx.report_with_fix(
                start,
                end,
                "A trailing decimal point can be confused with a dot.",
                Fix {
                    range: Span { start, end },
                    text: format!("{text}0"),
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
        assert!(lint(Box::new(NoFloatingDecimal), "var x = 0.5;").is_empty());
        assert!(lint(Box::new(NoFloatingDecimal), "var x = 1.0;").is_empty());
    }
    #[test]
    fn invalid_leading() {
        let d = lint(Box::new(NoFloatingDecimal), "var x = .5;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn fix_leading() {
        assert_fix(
            Box::new(NoFloatingDecimal),
            "var x = .5;",
            "var x = 0.5;",
        );
    }
    #[test]
    fn fix_trailing() {
        assert_fix(
            Box::new(NoFloatingDecimal),
            "var x = 1.;",
            "var x = 1.0;",
        );
    }
}
