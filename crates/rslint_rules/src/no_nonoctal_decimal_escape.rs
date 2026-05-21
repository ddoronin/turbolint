use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoNonOctalDecimalEscape;

impl Rule for NoNonOctalDecimalEscape {
    fn name(&self) -> &'static str {
        "no-nonoctal-decimal-escape"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "string" {
            let text = ctx.node_text(node);
            if text.contains("\\8") || text.contains("\\9") {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Don't use '\\8' and '\\9' escape sequences in string literals.",
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
        assert!(lint(Box::new(NoNonOctalDecimalEscape), r#"var x = "\1";"#).is_empty());
        assert!(lint(Box::new(NoNonOctalDecimalEscape), r#"var x = "hello";"#).is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoNonOctalDecimalEscape), r#"var x = "\8";"#);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-nonoctal-decimal-escape");
    }
}
