use crate::helpers::unwrap_parens;
use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoConstantCondition;

impl Rule for NoConstantCondition {
    fn name(&self) -> &'static str {
        "no-constant-condition"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "if_statement" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    check_condition(&cond, ctx, node);
                }
            }
            "while_statement" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    // Default checkLoops: "allExceptWhileTrue" — exempt while(true)
                    let inner = unwrap_parens(&cond);
                    if inner.kind() == "true" {
                        return;
                    }
                    check_condition(&cond, ctx, node);
                }
            }
            "do_statement" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    check_condition(&cond, ctx, node);
                }
            }
            "for_statement" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    check_condition(&cond, ctx, node);
                }
            }
            "ternary_expression" => {
                if let Some(cond) = node.child_by_field_name("condition") {
                    if is_constant(&cond) {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Unexpected constant condition.",
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

fn check_condition(cond: &Node, ctx: &RuleContext, report_node: &Node) {
    if is_constant(cond) {
        ctx.report(
            report_node.start_byte() as u32,
            report_node.end_byte() as u32,
            "Unexpected constant condition.",
        );
    }
}

fn is_constant(node: &Node) -> bool {
    match node.kind() {
        "true" | "false" | "number" | "string" | "null" | "undefined" | "regex" => true,
        "template_string" => {
            let mut cursor = node.walk();
            let has_sub = node
                .named_children(&mut cursor)
                .any(|c| c.kind() == "template_substitution");
            !has_sub
        }
        "array" | "object" => true,
        "parenthesized_expression" => {
            node.named_child(0).is_some_and(|c| is_constant(&c))
        }
        "unary_expression" => {
            node.child_by_field_name("argument")
                .is_some_and(|arg| is_constant(&arg))
        }
        "binary_expression" => {
            node.child_by_field_name("left")
                .is_some_and(|l| is_constant(&l))
                && node
                    .child_by_field_name("right")
                    .is_some_and(|r| is_constant(&r))
        }
        "sequence_expression" => {
            let count = node.named_child_count();
            count > 0
                && node
                    .named_child(count - 1)
                    .is_some_and(|last| is_constant(&last))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoConstantCondition), "if (x) {}").is_empty());
    }
    #[test]
    fn valid_while_true() {
        assert!(lint(Box::new(NoConstantCondition), "while (true) {}").is_empty());
    }
    #[test]
    fn invalid_true() {
        let d = lint(Box::new(NoConstantCondition), "if (true) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_number() {
        let d = lint(Box::new(NoConstantCondition), "if (1) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_not_false() {
        let d = lint(Box::new(NoConstantCondition), "if (!false) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_binary_constants() {
        let d = lint(Box::new(NoConstantCondition), "if (1 + 2) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_while_false() {
        let d = lint(Box::new(NoConstantCondition), "while (false) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_for_constant() {
        let d = lint(Box::new(NoConstantCondition), "for (;true;) {}");
        assert_eq!(d.len(), 1);
    }
}
