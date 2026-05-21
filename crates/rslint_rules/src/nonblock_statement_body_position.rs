use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NonblockStatementBodyPosition;
impl Rule for NonblockStatementBodyPosition {
    fn name(&self) -> &'static str {
        "nonblock-statement-body-position"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, _node: &Node, _ctx: &RuleContext) { /* deprecated formatting rule */
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn no_op() {
        assert!(lint(Box::new(NonblockStatementBodyPosition), "var x = 1;").is_empty());
    }
}
