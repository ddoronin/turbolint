use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessComputedKey;

impl Rule for NoUselessComputedKey {
    fn name(&self) -> &'static str {
        "no-useless-computed-key"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "computed_property_name" {
            return;
        }
        if let Some(inner) = node.named_child(0) {
            if inner.kind() == "string" || inner.kind() == "number" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Unnecessarily computed property key.",
                );
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
        assert!(lint(Box::new(NoUselessComputedKey), "var obj = { a: 1 };").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUselessComputedKey), r#"var obj = { ["a"]: 1 };"#);
        assert_eq!(d.len(), 1);
    }
}
