use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoDuplicateCase;

impl Rule for NoDuplicateCase {
    fn name(&self) -> &'static str {
        "no-duplicate-case"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "switch_body" {
            return;
        }
        let mut cases: Vec<String> = Vec::new();
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "switch_case" {
                if let Some(value) = child.child_by_field_name("value") {
                    let text = ctx.node_text(&value).to_string();
                    if cases.contains(&text) {
                        ctx.report(
                            value.start_byte() as u32,
                            value.end_byte() as u32,
                            format!("Duplicate case label '{}'.", text),
                        );
                    } else {
                        cases.push(text);
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
        assert!(lint(
            Box::new(NoDuplicateCase),
            "switch(x) { case 1: break; case 2: break; }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoDuplicateCase),
            "switch(x) { case 1: break; case 1: break; }",
        );
        assert_eq!(d.len(), 1);
    }
}
