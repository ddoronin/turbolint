use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct RequireUnicodeRegexp;

impl Rule for RequireUnicodeRegexp {
    fn name(&self) -> &'static str {
        "require-unicode-regexp"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "regex" {
            return;
        }
        let text = ctx.node_text(node);
        // Extract flags after the last /
        if let Some(last_slash) = text.rfind('/') {
            let flags = &text[last_slash + 1..];
            if !flags.contains('u') && !flags.contains('v') {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Use the 'u' or 'v' flag.",
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
        assert!(lint(Box::new(RequireUnicodeRegexp), "var re = /foo/u;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(RequireUnicodeRegexp), "var re = /foo/;");
        assert_eq!(d.len(), 1);
    }
}
