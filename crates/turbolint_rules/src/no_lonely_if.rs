use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoLonelyIf;

impl Rule for NoLonelyIf {
    fn name(&self) -> &'static str {
        "no-lonely-if"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "if_statement" {
            return;
        }
        // Check if this if is the only statement inside an else block
        if let Some(parent) = node.parent() {
            // Parent is statement_block inside else_clause
            if parent.kind() == "statement_block" && parent.named_child_count() == 1 {
                if let Some(grandparent) = parent.parent() {
                    if grandparent.kind() == "else_clause" {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Unexpected if as the only statement in an else block.",
                        );
                    }
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
        assert!(lint(Box::new(NoLonelyIf), "if (a) {} else if (b) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoLonelyIf), "if (a) {} else { if (b) {} }");
        assert_eq!(d.len(), 1);
    }
}
