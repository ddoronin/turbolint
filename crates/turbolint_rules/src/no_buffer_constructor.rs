use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoBufferConstructor;

impl Rule for NoBufferConstructor {
    fn name(&self) -> &'static str {
        "no-buffer-constructor"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        let is_buffer = match node.kind() {
            "new_expression" => node
                .child_by_field_name("constructor")
                .is_some_and(|c| ctx.node_text(&c) == "Buffer"),
            "call_expression" => node
                .child_by_field_name("function")
                .is_some_and(|f| f.kind() == "identifier" && ctx.node_text(&f) == "Buffer"),
            _ => false,
        };
        if is_buffer {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Use Buffer.alloc() or Buffer.from() instead.",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoBufferConstructor), "var x = Buffer.alloc(10);").is_empty());
    }
    #[test]
    fn invalid_new() {
        let d = lint(Box::new(NoBufferConstructor), "var x = new Buffer(10);");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_call() {
        let d = lint(Box::new(NoBufferConstructor), "var x = Buffer(10);");
        assert_eq!(d.len(), 1);
    }
}
