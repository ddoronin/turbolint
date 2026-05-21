use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoNewFunc;

impl Rule for NoNewFunc {
    fn name(&self) -> &'static str {
        "no-new-func"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "new_expression" {
            if let Some(constructor) = node.child_by_field_name("constructor") {
                if ctx.node_text(&constructor) == "Function" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "The Function constructor is eval.",
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
        assert!(lint(Box::new(NoNewFunc), "var x = function() {};").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoNewFunc),
            "var x = new Function('a', 'return a');",
        );
        assert_eq!(d.len(), 1);
    }
}
