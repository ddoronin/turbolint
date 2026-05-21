use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct Strict;
impl Rule for Strict {
    fn name(&self) -> &'static str {
        "strict"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, _node: &Node, _ctx: &RuleContext) {
        // Requires configuration (global/function/never mode). No-op placeholder.
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn no_op() {
        assert!(lint(Box::new(Strict), "var x = 1;").is_empty());
    }
}
