use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct DefaultCaseLast;

impl Rule for DefaultCaseLast {
    fn name(&self) -> &'static str {
        "default-case-last"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "switch_body" {
            return;
        }
        let mut cursor = node.walk();
        let cases: Vec<_> = node.named_children(&mut cursor).collect();
        for (i, case) in cases.iter().enumerate() {
            if case.kind() == "switch_default" && i != cases.len() - 1 {
                ctx.report(
                    case.start_byte() as u32,
                    case.end_byte() as u32,
                    "Default clause should be the last clause.",
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
        assert!(lint(
            Box::new(DefaultCaseLast),
            "switch(x) { case 1: break; default: break; }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(DefaultCaseLast),
            "switch(x) { default: break; case 1: break; }",
        );
        assert_eq!(d.len(), 1);
    }
}
