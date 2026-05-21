use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoThrowLiteral;

impl Rule for NoThrowLiteral {
    fn name(&self) -> &'static str {
        "no-throw-literal"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "throw_statement" {
            return;
        }
        if let Some(arg) = node.named_child(0) {
            if matches!(
                arg.kind(),
                "string" | "number" | "true" | "false" | "null" | "undefined"
            ) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Expected an error object to be thrown.",
                );
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
        assert!(lint(Box::new(NoThrowLiteral), "throw new Error('msg');").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoThrowLiteral), "throw 'error';");
        assert_eq!(d.len(), 1);
    }
}
