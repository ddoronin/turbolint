use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoExtraBooleanCast;

impl Rule for NoExtraBooleanCast {
    fn name(&self) -> &'static str {
        "no-extra-boolean-cast"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "unary_expression" {
            return;
        }
        let op = match node.child_by_field_name("operator") {
            Some(o) => o,
            None => return,
        };
        if ctx.node_text(&op) != "!" {
            return;
        }
        // Check for !! (double negation)
        let arg = match node.child_by_field_name("argument") {
            Some(a) => a,
            None => return,
        };
        if arg.kind() == "unary_expression" {
            if let Some(inner_op) = arg.child_by_field_name("operator") {
                if ctx.node_text(&inner_op) == "!" {
                    // Check if in a boolean context
                    if let Some(parent) = node.parent() {
                        if matches!(
                            parent.kind(),
                            "if_statement"
                                | "while_statement"
                                | "do_statement"
                                | "for_statement"
                                | "ternary_expression"
                        ) || (parent.kind() == "parenthesized_expression"
                            && parent.parent().is_some_and(|gp| {
                                matches!(
                                    gp.kind(),
                                    "if_statement" | "while_statement" | "do_statement"
                                )
                            }))
                        {
                            ctx.report(
                                node.start_byte() as u32,
                                node.end_byte() as u32,
                                "Redundant double negation.",
                            );
                        }
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
        assert!(lint(Box::new(NoExtraBooleanCast), "if (foo) {}").is_empty());
        assert!(lint(Box::new(NoExtraBooleanCast), "var x = !!foo;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoExtraBooleanCast), "if (!!foo) {}");
        assert_eq!(d.len(), 1);
    }
}
