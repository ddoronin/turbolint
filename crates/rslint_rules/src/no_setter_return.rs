use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoSetterReturn;

impl Rule for NoSetterReturn {
    fn name(&self) -> &'static str {
        "no-setter-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "return_statement" || node.named_child_count() == 0 {
            return;
        }
        // Walk up to find if inside a setter
        let mut current = node.parent();
        while let Some(p) = current {
            if p.kind() == "method_definition" {
                let mut cursor = p.walk();
                for child in p.children(&mut cursor) {
                    if child.kind() == "set" {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Setter cannot return a value.",
                        );
                        return;
                    }
                }
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
            Box::new(NoSetterReturn),
            "var obj = { set foo(val) { this._foo = val; } };"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoSetterReturn),
            "var obj = { set foo(val) { return val; } };",
        );
        assert_eq!(d.len(), 1);
    }
}
