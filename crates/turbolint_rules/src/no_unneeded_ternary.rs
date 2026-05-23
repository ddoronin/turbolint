use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::{Fix, Severity, Span};
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
        let condition = match node.child_by_field_name("condition") {
            Some(c) => c,
            None => return,
        };
        let cons_text = ctx.node_text(&consequence);
        let alt_text = ctx.node_text(&alternative);
        let start = node.start_byte() as u32;
        let end = node.end_byte() as u32;

        if cons_text == "true" && alt_text == "false" {
            // a ? true : false → a
            let cond_text = ctx.node_text(&condition);
            ctx.report_with_fix(
                start,
                end,
                "Unnecessary use of boolean literals in conditional expression.",
                Fix {
                    range: Span { start, end },
                    text: cond_text.to_string(),
                },
            );
        } else if cons_text == "false" && alt_text == "true" {
            // a ? false : true → !a
            let cond_text = ctx.node_text(&condition);
            let needs_parens = condition.kind() == "binary_expression"
                || condition.kind() == "ternary_expression"
                || condition.kind() == "assignment_expression"
                || condition.kind() == "sequence_expression";
            let fix_text = if needs_parens {
                format!("!({})", cond_text)
            } else {
                format!("!{}", cond_text)
            };
            ctx.report_with_fix(
                start,
                end,
                "Unnecessary use of boolean literals in conditional expression.",
                Fix {
                    range: Span { start, end },
                    text: fix_text,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    use turbolint_core::Linter;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUnneededTernary), "var x = a ? b : c;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUnneededTernary), "var x = a ? true : false;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn autofix_true_false() {
        let linter = Linter::new(vec![Box::new(NoUnneededTernary)]);
        let result = linter.lint_and_fix("var x = a ? true : false;");
        assert!(result.fixed);
        assert_eq!(result.output, "var x = a;");
    }
    #[test]
    fn autofix_false_true() {
        let linter = Linter::new(vec![Box::new(NoUnneededTernary)]);
        let result = linter.lint_and_fix("var x = a ? false : true;");
        assert!(result.fixed);
        assert_eq!(result.output, "var x = !a;");
    }
    #[test]
    fn autofix_false_true_complex_condition() {
        let linter = Linter::new(vec![Box::new(NoUnneededTernary)]);
        let result = linter.lint_and_fix("var x = a || b ? false : true;");
        assert!(result.fixed);
        assert_eq!(result.output, "var x = !(a || b);");
    }
}
