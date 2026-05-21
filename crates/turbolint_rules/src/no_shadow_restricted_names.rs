use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct NoShadowRestrictedNames;
impl Rule for NoShadowRestrictedNames {
    fn name(&self) -> &'static str {
        "no-shadow-restricted-names"
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
        assert!(lint(Box::new(NoShadowRestrictedNames), "var x = 1;").is_empty());
    }
}
