use rslint_core::ast_helpers::find_ancestor;
use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoConstructorReturn;

impl Rule for NoConstructorReturn {
    fn name(&self) -> &'static str {
        "no-constructor-return"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "return_statement" {
            return;
        }
        // Only report if return has a value (named child count > 0)
        if node.named_child_count() == 0 {
            return;
        }
        // Check if inside a constructor method
        if let Some(method) = find_ancestor(node, "method_definition") {
            if let Some(name) = method.child_by_field_name("name") {
                if ctx.node_text(&name) == "constructor" {
                    // Make sure we're not inside a nested function
                    let mut current = node.parent();
                    while let Some(p) = current {
                        if p.id() == method.id() {
                            ctx.report(
                                node.start_byte() as u32,
                                node.end_byte() as u32,
                                "Unexpected return statement in constructor.",
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
                            return; // nested function, not the constructor
                        }
                        current = p.parent();
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
        assert!(lint(
            Box::new(NoConstructorReturn),
            "class Foo { constructor() { this.x = 1; } }"
        )
        .is_empty());
        // Return without value is ok
        assert!(lint(
            Box::new(NoConstructorReturn),
            "class Foo { constructor() { return; } }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoConstructorReturn),
            "class Foo { constructor() { return 42; } }",
        );
        assert_eq!(d.len(), 1);
    }
}
