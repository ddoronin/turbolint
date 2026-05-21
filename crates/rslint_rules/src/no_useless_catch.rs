use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessCatch;

impl Rule for NoUselessCatch {
    fn name(&self) -> &'static str {
        "no-useless-catch"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "catch_clause" {
            return;
        }
        let param = match node.child_by_field_name("parameter") {
            Some(p) => p,
            None => return,
        };
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if body.named_child_count() != 1 {
            return;
        }
        if let Some(stmt) = body.named_child(0) {
            if stmt.kind() == "throw_statement" {
                if let Some(arg) = stmt.named_child(0) {
                    if ctx.node_text(&arg) == ctx.node_text(&param) {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Unnecessary catch clause.",
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
        assert!(lint(
            Box::new(NoUselessCatch),
            "try { foo(); } catch (e) { handle(e); }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoUselessCatch),
            "try { foo(); } catch (e) { throw e; }",
        );
        assert_eq!(d.len(), 1);
    }
}
