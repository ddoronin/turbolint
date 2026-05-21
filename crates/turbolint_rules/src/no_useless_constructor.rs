use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessConstructor;

impl Rule for NoUselessConstructor {
    fn name(&self) -> &'static str {
        "no-useless-constructor"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "method_definition" {
            return;
        }
        if let Some(name) = node.child_by_field_name("name") {
            if ctx.node_text(&name) != "constructor" {
                return;
            }
        } else {
            return;
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if body.named_child_count() == 0 {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Useless constructor.",
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
        assert!(lint(
            Box::new(NoUselessConstructor),
            "class Foo { constructor() { this.x = 1; } }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoUselessConstructor),
            "class Foo { constructor() {} }",
        );
        assert_eq!(d.len(), 1);
    }
}
