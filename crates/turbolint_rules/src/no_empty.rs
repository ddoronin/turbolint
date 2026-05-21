use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoEmpty;

impl Rule for NoEmpty {
    fn name(&self) -> &'static str {
        "no-empty"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "statement_block" && node.named_child_count() == 0 {
            // Allow empty catch blocks by default
            if let Some(parent) = node.parent() {
                if parent.kind() == "catch_clause" {
                    return;
                }
            }
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Empty block statement.",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoEmpty), "if (foo) { bar(); }").is_empty());
        assert!(lint(Box::new(NoEmpty), "try { foo(); } catch (e) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoEmpty), "if (foo) {}");
        assert_eq!(d.len(), 1);
    }
}
