use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::{Fix, Severity, Span};
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoVar;

impl Rule for NoVar {
    fn name(&self) -> &'static str {
        "no-var"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "variable_declaration" {
            return;
        }
        let text = ctx.node_text(node);
        if text.starts_with("var ") || text.starts_with("var\t") || text.starts_with("var\n") {
            let start = node.start_byte() as u32;
            ctx.report_with_fix(
                start,
                node.end_byte() as u32,
                "Unexpected var, use let or const instead.",
                Fix {
                    range: Span {
                        start,
                        end: start + 3,
                    },
                    text: "let".to_string(),
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
        assert!(lint(Box::new(NoVar), "let x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoVar), "var x = 1;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn fix_var_to_let() {
        assert_fix(Box::new(NoVar), "var x = 1;", "let x = 1;");
    }
}
