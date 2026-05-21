use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct NoInnerDeclarations;
impl Rule for NoInnerDeclarations {
    fn name(&self) -> &'static str {
        "no-inner-declarations"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "function_declaration" {
            return;
        }
        if let Some(parent) = node.parent() {
            if matches!(parent.kind(), "program" | "export_statement") {
                return;
            }
            if parent.kind() == "statement_block" {
                if let Some(gp) = parent.parent() {
                    if matches!(
                        gp.kind(),
                        "function_declaration"
                            | "function_expression"
                            | "arrow_function"
                            | "method_definition"
                            | "program"
                    ) {
                        return;
                    }
                }
            }
        }
        ctx.report(
            node.start_byte() as u32,
            node.end_byte() as u32,
            "Move function declaration to program or function body root.",
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;
    #[test]
    fn valid() {
        assert!(lint(Box::new(NoInnerDeclarations), "function foo() {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoInnerDeclarations),
            "if (x) { function foo() {} }",
        );
        assert_eq!(d.len(), 1);
    }
}
