use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct NoMixedSpacesAndTabs;
impl Rule for NoMixedSpacesAndTabs {
    fn name(&self) -> &'static str {
        "no-mixed-spaces-and-tabs"
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
        assert!(lint(Box::new(NoMixedSpacesAndTabs), "var x = 1;").is_empty());
    }
}
