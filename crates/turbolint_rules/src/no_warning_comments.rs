use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoWarningComments;

const DEFAULT_TERMS: &[&str] = &["todo", "fixme", "xxx"];

impl Rule for NoWarningComments {
    fn name(&self) -> &'static str {
        "no-warning-comments"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "comment" {
            return;
        }
        let text = ctx.node_text(node).to_lowercase();
        for term in DEFAULT_TERMS {
            if text.contains(term) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!("Unexpected '{}' comment.", term),
                );
                return;
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
        assert!(lint(Box::new(NoWarningComments), "// this is fine\nvar x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoWarningComments), "// TODO: fix this\nvar x = 1;");
        assert_eq!(d.len(), 1);
    }
}
