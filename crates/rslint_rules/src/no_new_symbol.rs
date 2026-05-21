use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoNewSymbol;

impl Rule for NoNewSymbol {
    fn name(&self) -> &'static str {
        "no-new-symbol"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, _node: &Node, _ctx: &RuleContext) {
        // Deprecated: superseded by no-new-native-nonconstructor which covers Symbol + BigInt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn deprecated_noop() {
        // Deprecated in favor of no-new-native-nonconstructor
        assert!(lint(Box::new(NoNewSymbol), "var s = new Symbol('foo');").is_empty());
    }
}
