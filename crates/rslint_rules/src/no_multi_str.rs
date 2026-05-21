use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoMultiStr;

impl Rule for NoMultiStr {
    fn name(&self) -> &'static str {
        "no-multi-str"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "string" {
            let text = ctx.node_text(node);
            if text.contains("\\\n") || text.contains("\\\r") {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Multiline support is limited to browsers supporting ES5 (or older).",
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
        assert!(lint(Box::new(NoMultiStr), r#"var x = "normal string";"#).is_empty());
    }
    #[test]
    fn invalid_backslash_newline() {
        let d = lint(Box::new(NoMultiStr), "var x = \"line1\\\nline2\";");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-multi-str");
    }
}
