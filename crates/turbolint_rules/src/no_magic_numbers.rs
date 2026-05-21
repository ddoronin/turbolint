use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoMagicNumbers;

const ALLOWED: &[&str] = &["-1", "0", "1", "2"];

impl Rule for NoMagicNumbers {
    fn name(&self) -> &'static str {
        "no-magic-numbers"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "number" {
            return;
        }
        let text = ctx.node_text(node);
        // Check if parent is a unary_expression with - (for negative numbers)
        let effective_text = if let Some(parent) = node.parent() {
            if parent.kind() == "unary_expression" {
                if let Some(op) = parent.child_by_field_name("operator") {
                    if ctx.node_text(&op) == "-" {
                        format!("-{}", text)
                    } else {
                        text.to_string()
                    }
                } else {
                    text.to_string()
                }
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        };

        if ALLOWED.contains(&effective_text.as_str()) {
            return;
        }
        // Allow in variable declarations
        if let Some(parent) = node.parent() {
            if parent.kind() == "variable_declarator" {
                return;
            }
            if parent.kind() == "unary_expression" {
                if let Some(gp) = parent.parent() {
                    if gp.kind() == "variable_declarator" {
                        return;
                    }
                }
            }
        }
        ctx.report(
            node.start_byte() as u32,
            node.end_byte() as u32,
            format!("No magic number: {}.", text),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoMagicNumbers), "var x = 1;").is_empty());
        assert!(lint(Box::new(NoMagicNumbers), "var x = 0;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoMagicNumbers), "if (x === 42) {}");
        assert_eq!(d.len(), 1);
    }
}
