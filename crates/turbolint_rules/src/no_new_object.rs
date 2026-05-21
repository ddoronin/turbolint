use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoNewObject;

impl Rule for NoNewObject {
    fn name(&self) -> &'static str {
        "no-new-object"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "new_expression" {
            if let Some(constructor) = node.child_by_field_name("constructor") {
                if ctx.node_text(&constructor) == "Object" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "The object literal notation {} is preferable.",
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
        assert!(lint(Box::new(NoNewObject), "var x = {};").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoNewObject), "var x = new Object();");
        assert_eq!(d.len(), 1);
    }
}
