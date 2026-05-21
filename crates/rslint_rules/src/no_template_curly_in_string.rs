use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoTemplateCurlyInString;

impl Rule for NoTemplateCurlyInString {
    fn name(&self) -> &'static str {
        "no-template-curly-in-string"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "string" {
            let text = ctx.node_text(node);
            if text.contains("${") {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Unexpected template string expression.",
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
        assert!(lint(
            Box::new(NoTemplateCurlyInString),
            "var x = `hello ${name}`;"
        )
        .is_empty());
        assert!(lint(Box::new(NoTemplateCurlyInString), "var x = 'hello';").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoTemplateCurlyInString),
            "var x = \"Hello ${name}\";",
        );
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-template-curly-in-string");
    }
}
