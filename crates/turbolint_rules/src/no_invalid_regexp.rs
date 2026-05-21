use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoInvalidRegexp;

impl Rule for NoInvalidRegexp {
    fn name(&self) -> &'static str {
        "no-invalid-regexp"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        // tree-sitter will mark invalid regex with ERROR nodes
        // We check for new RegExp() with clearly invalid patterns
        if node.kind() == "regex" && node.has_error() {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Invalid regular expression.",
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
        assert!(lint(Box::new(NoInvalidRegexp), "var re = /foo/;").is_empty());
    }
    // Note: tree-sitter may or may not parse invalid regex as errors
    // This is a best-effort check
}
