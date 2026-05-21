use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoFloatingDecimal;

impl Rule for NoFloatingDecimal {
    fn name(&self) -> &'static str {
        "no-floating-decimal"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "number" {
            return;
        }
        let text = ctx.node_text(node);
        if text.starts_with('.') {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "A leading decimal point can be confused with a dot.",
            );
        } else if text.ends_with('.') {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "A trailing decimal point can be confused with a dot.",
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
        assert!(lint(Box::new(NoFloatingDecimal), "var x = 0.5;").is_empty());
        assert!(lint(Box::new(NoFloatingDecimal), "var x = 1.0;").is_empty());
    }
    #[test]
    fn invalid_leading() {
        let d = lint(Box::new(NoFloatingDecimal), "var x = .5;");
        assert_eq!(d.len(), 1);
    }
}
