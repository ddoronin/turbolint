use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct PreferTemplate;
impl Rule for PreferTemplate {
    fn name(&self) -> &'static str {
        "prefer-template"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "binary_expression" {
            return;
        }
        let op = match node.child_by_field_name("operator") {
            Some(o) => o,
            None => return,
        };
        if ctx.node_text(&op) != "+" {
            return;
        }
        let left = match node.child_by_field_name("left") {
            Some(l) => l,
            None => return,
        };
        let right = match node.child_by_field_name("right") {
            Some(r) => r,
            None => return,
        };
        if (left.kind() == "string" || right.kind() == "string")
            && (left.kind() != "string" || right.kind() != "string")
        {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected string concatenation. Prefer template literals.",
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
        assert!(lint(Box::new(PreferTemplate), "var x = `hello ${name}`;").is_empty());
    }
}
