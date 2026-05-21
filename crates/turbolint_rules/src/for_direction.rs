use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct ForDirection;

impl Rule for ForDirection {
    fn name(&self) -> &'static str {
        "for-direction"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "for_statement" {
            return;
        }
        let condition = match node.child_by_field_name("condition") {
            Some(c) => c,
            None => return,
        };
        let increment = match node.child_by_field_name("increment") {
            Some(i) => i,
            None => return,
        };
        // Check if condition uses < or <=
        if condition.kind() != "binary_expression" {
            return;
        }
        let cond_op = match condition.child_by_field_name("operator") {
            Some(o) => o,
            None => return,
        };
        let cond_op_text = ctx.node_text(&cond_op);
        let cond_var = match condition.child_by_field_name("left") {
            Some(l) => ctx.node_text(&l).to_string(),
            None => return,
        };

        let wrong_direction = match cond_op_text {
            "<" | "<=" => is_decrementing(&increment, &cond_var, ctx),
            ">" | ">=" => is_incrementing(&increment, &cond_var, ctx),
            _ => false,
        };

        if wrong_direction {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "The update clause in this loop moves the variable in the wrong direction.",
            );
        }
    }
}

fn is_decrementing(node: &Node, var_name: &str, ctx: &RuleContext) -> bool {
    if node.kind() == "update_expression" {
        if let Some(op) = node.child_by_field_name("operator") {
            if let Some(arg) = node.child_by_field_name("argument") {
                return ctx.node_text(&op) == "--" && ctx.node_text(&arg) == var_name;
            }
        }
    }
    false
}

fn is_incrementing(node: &Node, var_name: &str, ctx: &RuleContext) -> bool {
    if node.kind() == "update_expression" {
        if let Some(op) = node.child_by_field_name("operator") {
            if let Some(arg) = node.child_by_field_name("argument") {
                return ctx.node_text(&op) == "++" && ctx.node_text(&arg) == var_name;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(ForDirection), "for (var i = 0; i < 10; i++) {}").is_empty());
        assert!(lint(Box::new(ForDirection), "for (var i = 10; i > 0; i--) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(ForDirection), "for (var i = 0; i < 10; i--) {}");
        assert_eq!(d.len(), 1);
    }
}
