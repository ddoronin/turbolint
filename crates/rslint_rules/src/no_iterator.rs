use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoIterator;

impl Rule for NoIterator {
    fn name(&self) -> &'static str {
        "no-iterator"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "member_expression" {
            if let Some(prop) = node.child_by_field_name("property") {
                if ctx.node_text(&prop) == "__iterator__" {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Reserved name '__iterator__'.",
                    );
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
        assert!(lint(Box::new(NoIterator), "var x = obj.iterator;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoIterator), "var x = obj.__iterator__;");
        assert_eq!(d.len(), 1);
    }
}
