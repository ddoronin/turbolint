use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoCondAssign;

impl Rule for NoCondAssign {
    fn name(&self) -> &'static str {
        "no-cond-assign"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "assignment_expression" {
            return;
        }
        if let Some(parent) = node.parent() {
            // Check if inside a condition (parenthesized_expression of if/while/do/for)
            let in_condition = is_in_condition(&parent);
            if in_condition {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Expected a conditional expression and instead saw an assignment.",
                );
            }
        }
    }
}

fn is_in_condition(node: &Node) -> bool {
    let mut current = Some(*node);
    while let Some(n) = current {
        match n.kind() {
            "if_statement" | "while_statement" | "do_statement" | "for_statement" => {
                return true;
            }
            "parenthesized_expression" => {
                current = n.parent();
                continue;
            }
            _ => return false,
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
        assert!(lint(Box::new(NoCondAssign), "if (x === 1) {}").is_empty());
        assert!(lint(Box::new(NoCondAssign), "var x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoCondAssign), "if (x = 1) {}");
        assert_eq!(d.len(), 1);
    }
}
