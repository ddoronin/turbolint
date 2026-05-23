use crate::helpers::unwrap_parens;
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
        match node.kind() {
            // Destructuring: const { a } = foo?.bar
            "variable_declarator" => {
                if let Some(name) = node.child_by_field_name("name") {
                    if matches!(name.kind(), "object_pattern" | "array_pattern") {
                        if let Some(value) = node.child_by_field_name("value") {
                            report_if_optional_chain(&value, ctx);
                        }
                    }
                }
            }
            // Assignment destructuring: ({ a } = foo?.bar)
            "assignment_expression" => {
                if let Some(left) = node.child_by_field_name("left") {
                    if matches!(
                        left.kind(),
                        "object_pattern" | "array_pattern" | "object" | "array"
                    ) {
                        if let Some(right) = node.child_by_field_name("right") {
                            report_if_optional_chain(&right, ctx);
                        }
                    }
                }
            }
            // for (x of foo?.bar)
            "for_in_statement" => {
                let source = ctx.source_text();
                let node_text = &source[node.start_byte()..node.end_byte()];
                if node_text.contains(" of ") {
                    if let Some(right) = node.child_by_field_name("right") {
                        report_if_optional_chain(&right, ctx);
                    }
                }
            }
            // Spread: [...foo?.bar] but NOT {...foo?.bar}
            "spread_element" => {
                if let Some(parent) = node.parent() {
                    if parent.kind() != "object" && parent.kind() != "object_pattern" {
                        if let Some(arg) = node.named_child(0) {
                            report_if_optional_chain(&arg, ctx);
                        }
                    }
                }
            }
            // class X extends foo?.bar {}
            "class_heritage" => {
                if let Some(child) = node.named_child(0) {
                    report_if_optional_chain(&child, ctx);
                }
            }
            // new (foo?.bar)()
            "new_expression" => {
                if let Some(constructor) = node.child_by_field_name("constructor") {
                    report_if_optional_chain(&constructor, ctx);
                }
            }
            // (foo?.bar)() — call where callee is unwrapped optional chain
            "call_expression" => {
                if let Some(func) = node.child_by_field_name("function") {
                    if is_unwrapped_optional_chain(&func) {
                        let inner = unwrap_parens(&func);
                        report_node(ctx, &inner);
                    }
                }
            }
            // (foo?.bar).baz — member access on unwrapped optional chain
            "member_expression" => {
                if let Some(obj) = node.child_by_field_name("object") {
                    if !has_optional_chain(node) && is_unwrapped_optional_chain(&obj) {
                        let inner = unwrap_parens(&obj);
                        report_node(ctx, &inner);
                    }
                }
            }
            // Binary in/instanceof — only RIGHT side
            "binary_expression" => {
                if let Some(op) = node.child_by_field_name("operator") {
                    let op_text = ctx.node_text(&op);
                    if matches!(op_text, "in" | "instanceof") {
                        if let Some(right) = node.child_by_field_name("right") {
                            report_if_optional_chain(&right, ctx);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// If the node (after unwrapping parens) is an optional chain, report it.
fn report_if_optional_chain(node: &Node, ctx: &RuleContext) {
    let unwrapped = unwrap_parens(node);
    if is_optional_chain_expr(&unwrapped) {
        report_node(ctx, &unwrapped);
    }
}

fn report_node(ctx: &RuleContext, node: &Node) {
    ctx.report(
        node.start_byte() as u32,
        node.end_byte() as u32,
        "Unsafe usage of optional chaining.",
    );
}

fn is_optional_chain_expr(node: &Node) -> bool {
    matches!(node.kind(), "member_expression" | "call_expression") && has_optional_chain(node)
}

/// Check if an expression is an optional chain wrapped in parentheses.
fn is_unwrapped_optional_chain(node: &Node) -> bool {
    node.kind() == "parenthesized_expression" && is_optional_chain_expr(&unwrap_parens(node))
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
    fn valid_optional_access() {
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "var x = foo?.bar;").is_empty());
    }
    #[test]
    fn valid_comparison() {
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "var x = foo?.bar === 1;").is_empty());
    }
    #[test]
    fn valid_logical() {
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "var x = foo?.bar && true;").is_empty());
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "var x = foo?.bar ?? true;").is_empty());
    }
    #[test]
    fn valid_typeof() {
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "typeof foo?.bar;").is_empty());
    }
    #[test]
    fn valid_arithmetic_default() {
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "var x = foo?.bar + 1;").is_empty());
        assert!(lint(Box::new(NoUnsafeOptionalChaining), "var x = -foo?.bar;").is_empty());
    }
    #[test]
    fn valid_instanceof_left() {
        assert!(lint(
            Box::new(NoUnsafeOptionalChaining),
            "foo?.bar instanceof Baz;"
        )
        .is_empty());
    }
    #[test]
    fn valid_spread_in_object() {
        assert!(lint(
            Box::new(NoUnsafeOptionalChaining),
            "var x = {...foo?.bar};"
        )
        .is_empty());
    }
    #[test]
    fn invalid_destructuring() {
        let d = lint(
            Box::new(NoUnsafeOptionalChaining),
            "var { a } = foo?.bar;",
        );
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_array_destructuring() {
        let d = lint(
            Box::new(NoUnsafeOptionalChaining),
            "var [a] = foo?.bar;",
        );
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_spread_in_array() {
        let d = lint(
            Box::new(NoUnsafeOptionalChaining),
            "var x = [...foo?.bar];",
        );
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_instanceof_right() {
        let d = lint(
            Box::new(NoUnsafeOptionalChaining),
            "foo instanceof bar?.baz;",
        );
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_in_right() {
        let d = lint(
            Box::new(NoUnsafeOptionalChaining),
            "'a' in foo?.bar;",
        );
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_for_of() {
        let d = lint(
            Box::new(NoUnsafeOptionalChaining),
            "for (var x of foo?.bar) {}",
        );
        assert_eq!(d.len(), 1);
    }
}
