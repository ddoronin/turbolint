use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct OneVarDeclarationPerLine;
impl Rule for OneVarDeclarationPerLine {
    fn name(&self) -> &'static str {
        "one-var-declaration-per-line"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, _node: &Node, _ctx: &RuleContext) { /* needs config/options */
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn no_op() {
        assert!(lint(Box::new(OneVarDeclarationPerLine), "var x = 1;").is_empty());
    }
}
