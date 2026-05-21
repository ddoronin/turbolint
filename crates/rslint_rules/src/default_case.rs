use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct DefaultCase;
impl Rule for DefaultCase {
    fn name(&self) -> &'static str {
        "default-case"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "switch_body" {
            return;
        }
        let mut cursor = node.walk();
        let has_default = node
            .named_children(&mut cursor)
            .any(|c| c.kind() == "switch_default");
        if !has_default {
            // Check for "no default" comment
            let source = ctx.source_text();
            let text = &source[node.start_byte()..node.end_byte()];
            if text.contains("no default") {
                return;
            }
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Expected a default case.",
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
        assert!(lint(
            Box::new(DefaultCase),
            "switch(x) { case 1: break; default: break; }"
        )
        .is_empty());
    }
}
