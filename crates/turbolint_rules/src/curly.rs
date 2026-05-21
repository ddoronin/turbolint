use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct Curly;
impl Rule for Curly {
    fn name(&self) -> &'static str {
        "curly"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "if_statement" => {
                if let Some(body) = node.child_by_field_name("consequence") {
                    if body.kind() != "statement_block" {
                        ctx.report(
                            body.start_byte() as u32,
                            body.end_byte() as u32,
                            "Expected { after 'if' condition.",
                        );
                    }
                }
            }
            "for_statement" | "for_in_statement" | "while_statement" | "do_statement" => {
                if let Some(body) = node.child_by_field_name("body") {
                    if body.kind() != "statement_block" {
                        ctx.report(
                            body.start_byte() as u32,
                            body.end_byte() as u32,
                            "Expected { after control statement.",
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
        assert!(lint(Box::new(Curly), "if (x) { foo(); }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(Curly), "if (x) foo();");
        assert_eq!(d.len(), 1);
    }
}
