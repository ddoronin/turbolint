use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoRestrictedSyntax;

impl Rule for NoRestrictedSyntax {
    fn name(&self) -> &'static str {
        "no-restricted-syntax"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, _node: &Node, _ctx: &RuleContext) {
        // This rule requires configuration (selectors) to be useful.
        // Without options infrastructure, it's a no-op placeholder.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn no_op_without_config() {
        assert!(lint(Box::new(NoRestrictedSyntax), "var x = 1;").is_empty());
    }
}
