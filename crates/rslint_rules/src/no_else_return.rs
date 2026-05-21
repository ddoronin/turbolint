use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoElseReturn;
impl Rule for NoElseReturn {
    fn name(&self) -> &'static str {
        "no-else-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "if_statement" {
            return;
        }
        let consequence = match node.child_by_field_name("consequence") {
            Some(c) => c,
            None => return,
        };
        let _alternative = match node.child_by_field_name("alternative") {
            Some(a) => a,
            None => return,
        };
        // Check if consequence ends with return
        if consequence.kind() == "statement_block" {
            if let Some(last) =
                consequence.named_child(consequence.named_child_count().saturating_sub(1))
            {
                if last.kind() == "return_statement" {
                    ctx.report(
                        _alternative.start_byte() as u32,
                        _alternative.end_byte() as u32,
                        "Unnecessary 'else' after 'return'.",
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
        assert!(lint(
            Box::new(NoElseReturn),
            "function f() { if (x) { return; } foo(); }"
        )
        .is_empty());
    }
}
