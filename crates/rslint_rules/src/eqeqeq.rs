use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
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
        match op_text {
            "==" => {
                ctx.report(
                    op.start_byte() as u32,
                    op.end_byte() as u32,
                    "Expected '===' and instead saw '=='.",
                );
            }
            "!=" => {
                ctx.report(
                    op.start_byte() as u32,
                    op.end_byte() as u32,
                    "Expected '!==' and instead saw '!='.",
                );
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

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
}
