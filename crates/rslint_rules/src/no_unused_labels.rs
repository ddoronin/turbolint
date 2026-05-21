use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUnusedLabels;

impl Rule for NoUnusedLabels {
    fn name(&self) -> &'static str {
        "no-unused-labels"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "labeled_statement" {
            return;
        }
        let label = match node.child_by_field_name("label") {
            Some(l) => l,
            None => return,
        };
        let label_name = ctx.node_text(&label);
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if !has_label_reference(&body, label_name, ctx) {
            ctx.report(
                label.start_byte() as u32,
                label.end_byte() as u32,
                format!("'{}' is defined but never used.", label_name),
            );
        }
    }
}

fn has_label_reference(node: &Node, label: &str, ctx: &RuleContext) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if matches!(child.kind(), "break_statement" | "continue_statement") {
            if let Some(lbl) = child.child_by_field_name("label") {
                if ctx.node_text(&lbl) == label {
                    return true;
                }
            }
        }
        if has_label_reference(&child, label, ctx) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUnusedLabels), "outer: for (;;) { break outer; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUnusedLabels), "label: for (;;) { break; }");
        assert_eq!(d.len(), 1);
    }
}
