use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoVoid;

impl Rule for NoVoid {
    fn name(&self) -> &'static str {
        "no-void"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "unary_expression" {
            if let Some(op) = node.child_by_field_name("operator") {
                if ctx.node_text(&op) == "void" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Expected 'undefined' and instead saw 'void'.",
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
        assert!(lint(Box::new(NoVoid), "var x = undefined;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoVoid), "var x = void 0;");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-void");
    }
}
