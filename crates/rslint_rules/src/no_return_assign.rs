use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoReturnAssign;

impl Rule for NoReturnAssign {
    fn name(&self) -> &'static str {
        "no-return-assign"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "return_statement" {
            return;
        }
        if let Some(arg) = node.named_child(0) {
            if has_assignment(&arg) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Return statement should not contain assignment.",
                );
            }
        }
    }
}

fn has_assignment(node: &Node) -> bool {
    if node.kind() == "assignment_expression" {
        return true;
    }
    if node.kind() == "parenthesized_expression" {
        if let Some(inner) = node.named_child(0) {
            return has_assignment(&inner);
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
        assert!(lint(Box::new(NoReturnAssign), "function foo() { return x; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoReturnAssign), "function foo() { return x = 1; }");
        assert_eq!(d.len(), 1);
    }
}
