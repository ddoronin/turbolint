use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoUnreachableLoop;
impl Rule for NoUnreachableLoop {
    fn name(&self) -> &'static str {
        "no-unreachable-loop"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "for_statement" | "for_in_statement" | "while_statement" | "do_statement"
        ) {
            return;
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        // Check if body always breaks/returns on first iteration
        if body.kind() == "statement_block" {
            let mut cursor = body.walk();
            let children: Vec<_> = body.named_children(&mut cursor).collect();
            if let Some(last) = children.last() {
                if matches!(
                    last.kind(),
                    "return_statement" | "throw_statement" | "break_statement"
                ) {
                    // No continue - this loop will only execute once
                    if !has_continue(&body) {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "This loop will only execute once.",
                        );
                    }
                }
            }
        }
    }
}
fn has_continue(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "continue_statement" {
            return true;
        }
        if matches!(
            child.kind(),
            "function_declaration"
                | "function_expression"
                | "arrow_function"
                | "for_statement"
                | "for_in_statement"
                | "while_statement"
                | "do_statement"
        ) {
            continue;
        }
        if has_continue(&child) {
            return true;
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
        assert!(lint(
            Box::new(NoUnreachableLoop),
            "for (var i = 0; i < 10; i++) { console.log(i); }"
        )
        .is_empty());
    }
}
