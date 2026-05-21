use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoDeleteVar;

impl Rule for NoDeleteVar {
    fn name(&self) -> &'static str {
        "no-delete-var"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "unary_expression" {
            return;
        }
        if let Some(op) = node.child_by_field_name("operator") {
            if ctx.node_text(&op) != "delete" {
                return;
            }
            if let Some(arg) = node.child_by_field_name("argument") {
                if arg.kind() == "identifier" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Variables should not be deleted.",
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
        assert!(lint(Box::new(NoDeleteVar), "delete obj.prop;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoDeleteVar), "var x = 1; delete x;");
        assert_eq!(d.len(), 1);
    }
}
