use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoRegexSpaces;

impl Rule for NoRegexSpaces {
    fn name(&self) -> &'static str {
        "no-regex-spaces"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "regex" {
            return;
        }
        let text = ctx.node_text(node);
        // Find the pattern part (between first and last /)
        if let Some(pattern) = extract_regex_pattern(text) {
            if pattern.contains("  ") {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Spaces are hard to count. Use {2} or more.",
                );
            }
        }
    }
}

fn extract_regex_pattern(text: &str) -> Option<&str> {
    let text = text.strip_prefix('/')?;
    let last_slash = text.rfind('/')?;
    Some(&text[..last_slash])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoRegexSpaces), "var re = /foo bar/;").is_empty());
        assert!(lint(Box::new(NoRegexSpaces), "var re = /foo {2}bar/;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoRegexSpaces), "var re = /foo  bar/;");
        assert_eq!(d.len(), 1);
    }
}
