use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct FuncNames;
impl Rule for FuncNames {
    fn name(&self) -> &'static str {
        "func-names"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "function_expression" {
            return;
        }
        if node.child_by_field_name("name").is_none() {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected unnamed function.",
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
        assert!(lint(Box::new(FuncNames), "var x = function foo() {};").is_empty());
    }
}
