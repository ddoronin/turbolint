use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoObjectConstructor;

impl Rule for NoObjectConstructor {
    fn name(&self) -> &'static str {
        "no-object-constructor"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "call_expression" {
            if let Some(func) = node.child_by_field_name("function") {
                if func.kind() == "identifier" && ctx.node_text(&func) == "Object" {
                    // Only report if no arguments
                    if let Some(args) = node.child_by_field_name("arguments") {
                        if args.named_child_count() > 0 {
                            return;
                        }
                    }
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Disallow calls to the Object constructor without an argument.",
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
        assert!(lint(Box::new(NoObjectConstructor), "var x = {};").is_empty());
        assert!(lint(Box::new(NoObjectConstructor), "var x = Object(value);").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoObjectConstructor), "var x = Object();");
        assert_eq!(d.len(), 1);
    }
}
