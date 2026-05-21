use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUnsafeOptionalChaining;

impl Rule for NoUnsafeOptionalChaining {
    fn name(&self) -> &'static str {
        "no-unsafe-optional-chaining"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        // In tree-sitter, optional chaining is a member_expression with an optional_chain child
        if node.kind() != "member_expression" && node.kind() != "call_expression" {
            return;
        }
        if !has_optional_chain(node) {
            return;
        }
        if let Some(parent) = node.parent() {
            if matches!(
                parent.kind(),
                "binary_expression"
                    | "unary_expression"
                    | "new_expression"
                    | "spread_element"
                    | "for_in_statement"
            ) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Unsafe usage of optional chaining.",
                );
            }
        }
    }
}

fn has_optional_chain(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "optional_chain" || child.kind() == "?." {
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
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "var x = foo?.bar;").is_empty());
    }
    #[test]
    fn invalid_arithmetic() {
        let d = lint(Box::new(NoUnsafeOptionalChaining), "var x = foo?.bar + 1;");
        assert_eq!(d.len(), 1);
    }
}
