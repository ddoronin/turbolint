use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoFallthrough;
impl Rule for NoFallthrough {
    fn name(&self) -> &'static str {
        "no-fallthrough"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "switch_case" {
            return;
        }
        // Check if this case has no break/return/throw at end
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).collect();
        if children.is_empty() {
            return;
        }
        let last = children.last().unwrap();
        if matches!(
            last.kind(),
            "break_statement" | "return_statement" | "throw_statement" | "continue_statement"
        ) {
            return;
        }
        // Check if last is a block that ends with break/return/throw
        if last.kind() == "statement_block" {
            if let Some(last_in_block) =
                last.named_child(last.named_child_count().saturating_sub(1))
            {
                if matches!(
                    last_in_block.kind(),
                    "break_statement"
                        | "return_statement"
                        | "throw_statement"
                        | "continue_statement"
                ) {
                    return;
                }
            }
        }
        // Check if there's a next case (otherwise fallthrough doesn't matter)
        if let Some(next_sib) = node.next_named_sibling() {
            if matches!(next_sib.kind(), "switch_case" | "switch_default") {
                // Check for "falls through" comment
                let source = ctx.source_text();
                let between = &source[node.end_byte()..next_sib.start_byte()];
                if between.contains("falls through")
                    || between.contains("fallthrough")
                    || between.contains("fall through")
                {
                    return;
                }
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Expected a 'break' statement before 'case'.",
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
        assert!(lint(
            Box::new(NoFallthrough),
            "switch(x) { case 1: break; case 2: break; }"
        )
        .is_empty());
    }
}
