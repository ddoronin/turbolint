use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoPlusplus;

impl Rule for NoPlusplus {
    fn name(&self) -> &'static str {
        "no-plusplus"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "update_expression" {
            if let Some(op) = node.child_by_field_name("operator") {
                let op_text = ctx.node_text(&op);
                if op_text == "++" || op_text == "--" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        format!("Unary operator '{}' used.", op_text),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoPlusplus), "var x = 1; x += 1;").is_empty());
    }
    #[test]
    fn invalid_increment() {
        let d = lint(Box::new(NoPlusplus), "var x = 1; x++;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_decrement() {
        let d = lint(Box::new(NoPlusplus), "var x = 1; x--;");
        assert_eq!(d.len(), 1);
    }
}
