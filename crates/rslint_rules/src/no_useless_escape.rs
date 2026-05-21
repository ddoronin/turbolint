use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessEscape;

const VALID_STRING_ESCAPES: &[u8] = b"\\nrtbfv0xuU'\"";

impl Rule for NoUselessEscape {
    fn name(&self) -> &'static str {
        "no-useless-escape"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "string" {
            return;
        }
        let text = ctx.node_text(node);
        let bytes = text.as_bytes();
        if bytes.len() < 2 {
            return;
        }
        let quote = bytes[0];
        let mut i = 1;
        while i < bytes.len() - 1 {
            if bytes[i] == b'\\' && i + 1 < bytes.len() - 1 {
                let next = bytes[i + 1];
                if next == quote || VALID_STRING_ESCAPES.contains(&next) || next.is_ascii_digit() {
                    // valid escape
                } else {
                    ctx.report(
                        (node.start_byte() + i) as u32,
                        (node.start_byte() + i + 2) as u32,
                        format!("Unnecessary escape character: \\{}.", next as char),
                    );
                }
                i += 2;
                continue;
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
        assert!(lint(Box::new(NoUselessEscape), r#"var x = "hello\n";"#).is_empty());
        assert!(lint(Box::new(NoUselessEscape), r#"var x = "hello";"#).is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUselessEscape), r#"var x = "he\llo";"#);
        assert_eq!(d.len(), 1);
    }
}
