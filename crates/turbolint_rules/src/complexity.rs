use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct Complexity;

const DEFAULT_MAX: usize = 20;

impl Rule for Complexity {
    fn name(&self) -> &'static str {
        "complexity"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "function_declaration" | "function_expression" | "arrow_function" | "method_definition"
        ) {
            return;
        }
        let complexity = compute_complexity(node);
        if complexity > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "Function has a complexity of {}. Maximum allowed is {}.",
                    complexity, DEFAULT_MAX
                ),
            );
        }
    }
}

fn compute_complexity(node: &Node) -> usize {
    let mut complexity = 1; // base complexity
    add_complexity(node, &mut complexity, true);
    complexity
}

fn add_complexity(node: &Node, complexity: &mut usize, is_root: bool) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        // Don't enter nested functions
        if !is_root
            && matches!(
                child.kind(),
                "function_declaration"
                    | "function_expression"
                    | "arrow_function"
                    | "generator_function"
                    | "generator_function_declaration"
            )
        {
            continue;
        }
        match child.kind() {
            "if_statement" | "for_statement" | "for_in_statement" | "while_statement"
            | "do_statement" | "switch_case" | "catch_clause" | "ternary_expression" => {
                *complexity += 1;
            }
            "binary_expression" => {
                if let Some(op) = child.child_by_field_name("operator") {
                    let _ = op; // logical operators could add complexity
                }
            }
            _ => {}
        }
        add_complexity(&child, complexity, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(Complexity), "function foo() { return 1; }").is_empty());
    }
}
