use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoAsyncPromiseExecutor;

impl Rule for NoAsyncPromiseExecutor {
    fn name(&self) -> &'static str {
        "no-async-promise-executor"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "new_expression" {
            return;
        }
        let constructor = match node.child_by_field_name("constructor") {
            Some(c) => c,
            None => return,
        };
        if ctx.node_text(&constructor) != "Promise" {
            return;
        }
        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };
        if let Some(first_arg) = args.named_child(0) {
            let text = ctx.node_text(&first_arg);
            if text.starts_with("async") {
                ctx.report(
                    first_arg.start_byte() as u32,
                    first_arg.end_byte() as u32,
                    "Promise executor functions should not be async.",
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
            Box::new(NoAsyncPromiseExecutor),
            "new Promise(function(resolve) { resolve(); });"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoAsyncPromiseExecutor),
            "new Promise(async function(resolve) { resolve(); });",
        );
        assert_eq!(d.len(), 1);
    }
}
