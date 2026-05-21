use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct RequireAtomicUpdates;
impl Rule for RequireAtomicUpdates {
    fn name(&self) -> &'static str {
        "require-atomic-updates"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, _node: &Node, _ctx: &RuleContext) { /* requires scope analysis */
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn no_op() {
        assert!(lint(Box::new(RequireAtomicUpdates), "var x = 1;").is_empty());
    }
}
