use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoVar;
impl Rule for NoVar {
    fn name(&self) -> &'static str {
        "no-var"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "variable_declaration" {
            return;
        }
        let text = ctx.node_text(node);
        if text.starts_with("var ") || text.starts_with("var\t") || text.starts_with("var\n") {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected var, use let or const instead.",
            );
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn valid() {
        assert!(lint(Box::new(NoVar), "let x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoVar), "var x = 1;");
        assert_eq!(d.len(), 1);
    }
}
