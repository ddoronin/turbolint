use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct PreferRegexLiterals;
impl Rule for PreferRegexLiterals {
    fn name(&self) -> &'static str {
        "prefer-regex-literals"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "new_expression" {
            return;
        }
        if let Some(c) = node.child_by_field_name("constructor") {
            if ctx.node_text(&c) == "RegExp" {
                if let Some(args) = node.child_by_field_name("arguments") {
                    if let Some(first) = args.named_child(0) {
                        if first.kind() == "string" {
                            ctx.report(node.start_byte() as u32, node.end_byte() as u32, "Use a regular expression literal instead of the RegExp constructor.");
                        }
                    }
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
        assert!(lint(Box::new(PreferRegexLiterals), "var re = /foo/;").is_empty());
    }
}
