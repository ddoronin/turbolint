use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoScriptUrl;

impl Rule for NoScriptUrl {
    fn name(&self) -> &'static str {
        "no-script-url"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "string" {
            let text = ctx.node_text(node);
            // Remove surrounding quotes and check for javascript: prefix
            if text.len() > 2 {
                let inner = &text[1..text.len() - 1];
                if inner.to_lowercase().starts_with("javascript:") {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Script URL is a form of eval.",
                    );
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
        assert!(lint(Box::new(NoScriptUrl), r#"var x = "http://example.com";"#).is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoScriptUrl), r#"var x = "javascript:void(0)";"#);
        assert_eq!(d.len(), 1);
    }
}
