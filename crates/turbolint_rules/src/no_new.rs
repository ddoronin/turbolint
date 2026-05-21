use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoNew;

impl Rule for NoNew {
    fn name(&self) -> &'static str {
        "no-new"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "new_expression" {
            if let Some(parent) = node.parent() {
                if parent.kind() == "expression_statement" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Do not use 'new' for side effects.",
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoNew), "var x = new Foo();").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoNew), "new Foo();");
        assert_eq!(d.len(), 1);
    }
}
