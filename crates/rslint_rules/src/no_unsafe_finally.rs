use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUnsafeFinally;

const CONTROL_FLOW: &[&str] = &[
    "return_statement",
    "throw_statement",
    "break_statement",
    "continue_statement",
];

impl Rule for NoUnsafeFinally {
    fn name(&self) -> &'static str {
        "no-unsafe-finally"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !CONTROL_FLOW.contains(&node.kind()) {
            return;
        }
        // Check if directly inside a finally block (not inside a nested function)
        let mut current = node.parent();
        while let Some(p) = current {
            if p.kind() == "finally_clause" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!(
                        "Unsafe usage of {} in finally block.",
                        node.kind().replace('_', " ").trim_end_matches(" statement")
                    ),
                );
                return;
            }
            if matches!(
                p.kind(),
                "function_declaration"
                    | "function_expression"
                    | "arrow_function"
                    | "generator_function"
                    | "generator_function_declaration"
            ) {
                return;
            }
            // For break/continue, loops also boundary
            if matches!(node.kind(), "break_statement" | "continue_statement")
                && matches!(
                    p.kind(),
                    "for_statement"
                        | "for_in_statement"
                        | "while_statement"
                        | "do_statement"
                        | "switch_statement"
                )
            {
                return;
            }
            current = p.parent();
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
            Box::new(NoUnsafeFinally),
            "try { foo(); } finally { bar(); }"
        )
        .is_empty());
    }
    #[test]
    fn invalid_return() {
        let d = lint(
            Box::new(NoUnsafeFinally),
            "try { foo(); } finally { return 1; }",
        );
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_throw() {
        let d = lint(
            Box::new(NoUnsafeFinally),
            "try { foo(); } finally { throw new Error(); }",
        );
        assert_eq!(d.len(), 1);
    }
}
