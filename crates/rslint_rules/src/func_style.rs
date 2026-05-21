use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct FuncStyle;
impl Rule for FuncStyle {
    fn name(&self) -> &'static str {
        "func-style"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, _node: &Node, _ctx: &RuleContext) {
        // Default "expression" mode - needs options to be useful
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn no_op() {
        assert!(lint(Box::new(FuncStyle), "var x = 1;").is_empty());
    }
}
