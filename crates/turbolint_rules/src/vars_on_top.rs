use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct VarsOnTop;
impl Rule for VarsOnTop {
    fn name(&self) -> &'static str {
        "vars-on-top"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "variable_declaration" {
            return;
        }
        // Check if this is a var (not let/const)
        let text = ctx.node_text(node);
        if !text.starts_with("var ") {
            return;
        }
        if let Some(parent) = node.parent() {
            if matches!(parent.kind(), "program" | "statement_block") {
                let mut cursor = parent.walk();
                let mut past_non_var = false;
                for child in parent.named_children(&mut cursor) {
                    if child.id() == node.id() {
                        if past_non_var {
                            ctx.report(
                                node.start_byte() as u32,
                                node.end_byte() as u32,
                                "All 'var' declarations must be at the top.",
                            );
                        }
                        return;
                    }
                    if child.kind() == "variable_declaration" {
                        let ct = ctx.node_text(&child);
                        if ct.starts_with("var ") {
                            continue;
                        }
                    }
                    if child.kind() == "expression_statement" {
                        if let Some(expr) = child.named_child(0) {
                            if expr.kind() == "string" {
                                continue;
                            } // directive
                        }
                    }
                    past_non_var = true;
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
        assert!(lint(Box::new(VarsOnTop), "var x = 1;").is_empty());
    }
}
