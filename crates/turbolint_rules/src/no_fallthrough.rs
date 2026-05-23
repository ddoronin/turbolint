use crate::helpers::{has_fallthrough_comment, is_terminal};
use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
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
        if node.kind() != "switch_case" && node.kind() != "switch_default" {
            return;
        }
        let mut cursor = node.walk();
        let children: Vec<_> = node.named_children(&mut cursor).collect();
        let skip = if node.kind() == "switch_case" { 1 } else { 0 };
        let body_children: Vec<_> = children.iter().skip(skip).collect();
        if body_children.is_empty() {
            return;
        }
        let last = *body_children.last().unwrap();
        if is_terminal(&last) {
            return;
        }
        // Check if last is a block that ends with a terminal statement
        if last.kind() == "statement_block" {
            if let Some(last_in_block) =
                last.named_child(last.named_child_count().saturating_sub(1))
            {
                if is_terminal(&last_in_block) {
                    return;
                }
            }
        }
        // Check if there's a next case
        if let Some(next_sib) = node.next_named_sibling() {
            if matches!(next_sib.kind(), "switch_case" | "switch_default") {
                let source = ctx.source_text();
                // Check for fallthrough comment between cases or at end of case body
                let between = &source[node.end_byte()..next_sib.start_byte()];
                let trailing = &source[last.end_byte()..node.end_byte()];
                if has_fallthrough_comment(between) || has_fallthrough_comment(trailing) {
                    return;
                }
                let msg = if next_sib.kind() == "switch_default" {
                    "Expected a 'break' statement before 'default'."
                } else {
                    "Expected a 'break' statement before 'case'."
                };
                ctx.report(node.start_byte() as u32, node.end_byte() as u32, msg);
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
    #[test]
    fn valid_empty_case() {
        assert!(lint(
            Box::new(NoFallthrough),
            "switch(x) { case 1: case 2: break; }"
        )
        .is_empty());
    }
    #[test]
    fn valid_fallthrough_comment() {
        assert!(lint(
            Box::new(NoFallthrough),
            "switch(x) { case 1: foo(); // Falls Through\ncase 2: break; }"
        )
        .is_empty());
    }
    #[test]
    fn valid_fallthrough_comment_case_insensitive() {
        assert!(lint(
            Box::new(NoFallthrough),
            "switch(x) { case 1: foo(); // FALLS THROUGH\ncase 2: break; }"
        )
        .is_empty());
    }
    #[test]
    fn invalid_default_fallthrough() {
        let d = lint(
            Box::new(NoFallthrough),
            "switch(x) { case 1: foo(); default: break; }",
        );
        assert_eq!(d.len(), 1);
    }
}
