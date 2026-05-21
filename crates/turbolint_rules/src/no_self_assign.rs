use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct NoSelfAssign;
impl Rule for NoSelfAssign {
    fn name(&self) -> &'static str {
        "no-self-assign"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "assignment_expression" {
            return;
        }
        let left = match node.child_by_field_name("left") {
            Some(l) => l,
            None => return,
        };
        let right = match node.child_by_field_name("right") {
            Some(r) => r,
            None => return,
        };
        if ctx.node_text(&left) == ctx.node_text(&right) {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!("'{}' is assigned to itself.", ctx.node_text(&left)),
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
        assert!(lint(Box::new(NoSelfAssign), "a = b;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoSelfAssign), "a = a;");
        assert_eq!(d.len(), 1);
    }
}
