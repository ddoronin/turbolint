use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoMultiAssign;

impl Rule for NoMultiAssign {
    fn name(&self) -> &'static str {
        "no-multi-assign"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "assignment_expression" {
            return;
        }
        if let Some(right) = node.child_by_field_name("right") {
            if right.kind() == "assignment_expression" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Unexpected chained assignment.",
                );
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
        assert!(lint(Box::new(NoMultiAssign), "a = 1; b = 2;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoMultiAssign), "a = b = c = 1;");
        assert!(d.len() >= 1);
    }
}
