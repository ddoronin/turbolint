use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUndefInit;

impl Rule for NoUndefInit {
    fn name(&self) -> &'static str {
        "no-undef-init"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "variable_declarator" {
            return;
        }
        if let Some(value) = node.child_by_field_name("value") {
            if value.kind() == "undefined" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "It's not necessary to initialize to undefined.",
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
        assert!(lint(Box::new(NoUndefInit), "var x;").is_empty());
        assert!(lint(Box::new(NoUndefInit), "var x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUndefInit), "var x = undefined;");
        assert_eq!(d.len(), 1);
    }
}
