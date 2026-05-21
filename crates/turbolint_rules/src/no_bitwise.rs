use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoBitwise;

const BITWISE_OPS: &[&str] = &["&", "|", "^", "~", "<<", ">>", ">>>"];
const BITWISE_ASSIGN_OPS: &[&str] = &["&=", "|=", "^=", "<<=", ">>=", ">>>="];

impl Rule for NoBitwise {
    fn name(&self) -> &'static str {
        "no-bitwise"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "binary_expression" => {
                if let Some(op) = node.child_by_field_name("operator") {
                    let op_text = ctx.node_text(&op);
                    if BITWISE_OPS.contains(&op_text) {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            format!("Unexpected use of '{}'.", op_text),
                        );
                    }
                }
            }
            "unary_expression" => {
                if let Some(op) = node.child_by_field_name("operator") {
                    if ctx.node_text(&op) == "~" {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Unexpected use of '~'.",
                        );
                    }
                }
            }
            "augmented_assignment_expression" => {
                if let Some(op) = node.child_by_field_name("operator") {
                    let op_text = ctx.node_text(&op);
                    if BITWISE_ASSIGN_OPS.contains(&op_text) {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            format!("Unexpected use of '{}'.", op_text),
                        );
                    }
                }
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
        assert!(lint(Box::new(NoBitwise), "var x = a && b;").is_empty());
        assert!(lint(Box::new(NoBitwise), "var x = a || b;").is_empty());
    }
    #[test]
    fn invalid_or() {
        let d = lint(Box::new(NoBitwise), "var x = a | b;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_not() {
        let d = lint(Box::new(NoBitwise), "var x = ~a;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_assign() {
        let d = lint(Box::new(NoBitwise), "x |= 2;");
        assert_eq!(d.len(), 1);
    }
}
