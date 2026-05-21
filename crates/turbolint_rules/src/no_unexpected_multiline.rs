use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUnexpectedMultiline;

impl Rule for NoUnexpectedMultiline {
    fn name(&self) -> &'static str {
        "no-unexpected-multiline"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        // Detect call expressions or member access that starts on a new line
        // This catches things like:
        //   var a = b
        //   (x || y).doSomething()
        if node.kind() == "call_expression" {
            if let Some(args) = node.child_by_field_name("arguments") {
                if let Some(func) = node.child_by_field_name("function") {
                    if args.start_position().row > func.end_position().row {
                        ctx.report(
                            args.start_byte() as u32,
                            args.end_byte() as u32,
                            "Unexpected newline between function and ( of function call.",
                        );
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
        assert!(lint(Box::new(NoUnexpectedMultiline), "foo(1, 2);").is_empty());
    }
}
