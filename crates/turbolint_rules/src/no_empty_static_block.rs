use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoEmptyStaticBlock;

impl Rule for NoEmptyStaticBlock {
    fn name(&self) -> &'static str {
        "no-empty-static-block"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "class_static_block" {
            // The static block has a statement_block child; check if it's empty
            if let Some(body) = node.child_by_field_name("body") {
                if body.named_child_count() == 0 {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Unexpected empty static block.",
                    );
                }
            } else {
                // Fallback: look for statement_block child
                let mut cursor = node.walk();
                for child in node.named_children(&mut cursor) {
                    if child.kind() == "statement_block" && child.named_child_count() == 0 {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Unexpected empty static block.",
                        );
                        return;
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
            Box::new(NoEmptyStaticBlock),
            "class Foo { static { bar(); } }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoEmptyStaticBlock), "class Foo { static { } }");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-empty-static-block");
    }
}
