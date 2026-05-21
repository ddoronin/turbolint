use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct GuardForIn;

impl Rule for GuardForIn {
    fn name(&self) -> &'static str {
        "guard-for-in"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "for_in_statement" {
            return;
        }
        if let Some(body) = node.child_by_field_name("body") {
            // If body is a block, check if it starts with an if
            if body.kind() == "statement_block" {
                let mut cursor = body.walk();
                let first_stmt = body.named_children(&mut cursor).next();
                if let Some(stmt) = first_stmt {
                    if stmt.kind() == "if_statement" {
                        return; // guarded
                    }
                }
            } else if body.kind() == "if_statement" {
                return; // guarded without braces
            }
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "The body of a for-in should be wrapped in an if statement to filter unwanted properties from the prototype.",
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
        assert!(lint(
            Box::new(GuardForIn),
            "for (var key in obj) { if (obj.hasOwnProperty(key)) {} }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(GuardForIn), "for (var key in obj) { foo(key); }");
        assert_eq!(d.len(), 1);
    }
}
