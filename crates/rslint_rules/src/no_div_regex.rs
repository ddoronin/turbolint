use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoDivRegex;

impl Rule for NoDivRegex {
    fn name(&self) -> &'static str {
        "no-div-regex"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "regex" {
            let text = ctx.node_text(node);
            if text.starts_with("/=") {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "A regular expression literal can be confused with '/='.",
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
        assert!(lint(Box::new(NoDivRegex), "var re = /foo/;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoDivRegex), "var re = /=foo/;");
        assert_eq!(d.len(), 1);
    }
}
