use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoNewRequire;

impl Rule for NoNewRequire {
    fn name(&self) -> &'static str {
        "no-new-require"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "new_expression" {
            if let Some(constructor) = node.child_by_field_name("constructor") {
                if ctx.node_text(&constructor) == "require" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Unexpected use of new with require.",
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
        assert!(lint(Box::new(NoNewRequire), "var x = require('foo');").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoNewRequire), "var x = new require('foo');");
        assert_eq!(d.len(), 1);
    }
}
