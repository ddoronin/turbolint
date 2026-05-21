use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoOctal;

impl Rule for NoOctal {
    fn name(&self) -> &'static str {
        "no-octal"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "number" {
            let text = ctx.node_text(node);
            if is_legacy_octal(text) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Octal literals should not be used.",
                );
            }
        }
    }
}

fn is_legacy_octal(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'0' {
        return false;
    }
    // Exclude 0x, 0o, 0b, 0. prefixes and plain 0
    matches!(bytes[1], b'0'..=b'7')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoOctal), "var x = 1;").is_empty());
        assert!(lint(Box::new(NoOctal), "var x = 0x1;").is_empty());
        assert!(lint(Box::new(NoOctal), "var x = 0o1;").is_empty());
        assert!(lint(Box::new(NoOctal), "var x = 0b1;").is_empty());
        assert!(lint(Box::new(NoOctal), "var x = 0;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoOctal), "var x = 01;");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-octal");

        let d = lint(Box::new(NoOctal), "var x = 077;");
        assert_eq!(d.len(), 1);
    }
}
