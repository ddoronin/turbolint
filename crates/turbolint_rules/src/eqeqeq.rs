use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::{Fix, Severity, Span};
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct Eqeqeq;

impl Rule for Eqeqeq {
    fn name(&self) -> &'static str {
        "eqeqeq"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "binary_expression" {
            return;
        }
        let op = match node.child_by_field_name("operator") {
            Some(o) => o,
            None => return,
        };
        let op_text = ctx.node_text(&op);
        let start = op.start_byte() as u32;
        let end = op.end_byte() as u32;
        match op_text {
            "==" => {
                ctx.report_with_fix(
                    start,
                    end,
                    "Expected '===' and instead saw '=='.",
                    Fix {
                        range: Span { start, end },
                        text: "===".to_string(),
                    },
                );
            }
            "!=" => {
                ctx.report_with_fix(
                    start,
                    end,
                    "Expected '!==' and instead saw '!='.",
                    Fix {
                        range: Span { start, end },
                        text: "!==".to_string(),
                    },
                );
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{assert_fix, lint};

    #[test]
    fn valid() {
        assert!(lint(Box::new(Eqeqeq), "if (a === b) {}").is_empty());
        assert!(lint(Box::new(Eqeqeq), "if (a !== b) {}").is_empty());
    }
    #[test]
    fn invalid_eq() {
        let d = lint(Box::new(Eqeqeq), "if (a == b) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_neq() {
        let d = lint(Box::new(Eqeqeq), "if (a != b) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn fix_eq() {
        assert_fix(Box::new(Eqeqeq), "if (a == b) {}", "if (a === b) {}");
    }
    #[test]
    fn fix_neq() {
        assert_fix(Box::new(Eqeqeq), "if (a != b) {}", "if (a !== b) {}");
    }
}
