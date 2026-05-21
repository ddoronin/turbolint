use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct RequireYield;

impl Rule for RequireYield {
    fn name(&self) -> &'static str {
        "require-yield"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "generator_function" | "generator_function_declaration"
        ) {
            return;
        }
        if !has_yield(node) {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "This generator function does not have 'yield'.",
            );
        }
    }
}

fn has_yield(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "yield_expression" {
            return true;
        }
        // Don't descend into nested functions
        if matches!(
            child.kind(),
            "function_declaration"
                | "function_expression"
                | "arrow_function"
                | "generator_function"
                | "generator_function_declaration"
        ) {
            continue;
        }
        if has_yield(&child) {
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
        assert!(lint(Box::new(RequireYield), "function* gen() { yield 1; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(RequireYield), "function* gen() { return 1; }");
        assert_eq!(d.len(), 1);
    }
}
