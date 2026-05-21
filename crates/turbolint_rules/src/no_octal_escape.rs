use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoOctalEscape;

impl Rule for NoOctalEscape {
    fn name(&self) -> &'static str {
        "no-octal-escape"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "string" {
            return;
        }
        let text = ctx.node_text(node);
        let bytes = text.as_bytes();
        let mut i = 1; // skip opening quote
        while i < bytes.len().saturating_sub(1) {
            if bytes[i] == b'\\' && i + 1 < bytes.len() - 1 {
                let next = bytes[i + 1];
                if (b'0'..=b'7').contains(&next) {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Don't use octal escape sequences.",
                    );
                    return;
                }
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoOctalEscape), r#"var x = "hello";"#).is_empty());
        assert!(lint(Box::new(NoOctalEscape), r#"var x = "\n";"#).is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoOctalEscape), r#"var x = "\251";"#);
        assert_eq!(d.len(), 1);
    }
}
