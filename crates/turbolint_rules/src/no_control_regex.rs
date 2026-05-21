use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoControlRegex;

impl Rule for NoControlRegex {
    fn name(&self) -> &'static str {
        "no-control-regex"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "regex" {
            return;
        }
        let text = ctx.node_text(node);
        // Look for \x00-\x1f control characters in the pattern
        if text.contains("\\x0")
            || text.contains("\\x1")
            || text.contains("\\u000")
            || text.contains("\\u001")
        {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unexpected control character(s) in regular expression.",
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
        assert!(lint(Box::new(NoControlRegex), "var re = /foo/;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoControlRegex), r"var re = /\x00/;");
        assert_eq!(d.len(), 1);
    }
}
