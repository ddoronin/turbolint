use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoExtraLabel;

impl Rule for NoExtraLabel {
    fn name(&self) -> &'static str {
        "no-extra-label"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(node.kind(), "break_statement" | "continue_statement") {
            return;
        }
        let label = match node.child_by_field_name("label") {
            Some(l) => l,
            None => return,
        };
        // Check if the labeled loop/switch is the immediate enclosing one
        let mut current = node.parent();
        while let Some(p) = current {
            if matches!(
                p.kind(),
                "for_statement"
                    | "for_in_statement"
                    | "while_statement"
                    | "do_statement"
                    | "switch_statement"
            ) {
                // The innermost loop/switch doesn't need a label
                if let Some(gp) = p.parent() {
                    if gp.kind() == "labeled_statement" {
                        if let Some(lbl) = gp.child_by_field_name("label") {
                            if ctx.node_text(&lbl) == ctx.node_text(&label) {
                                ctx.report(
                                    label.start_byte() as u32,
                                    label.end_byte() as u32,
                                    format!(
                                        "This label '{}' is unnecessary.",
                                        ctx.node_text(&label)
                                    ),
                                );
                            }
                        }
                    }
                }
                return;
            }
            current = p.parent();
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
            Box::new(NoExtraLabel),
            "outer: for (;;) { for (;;) { break outer; } }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoExtraLabel), "label: for (;;) { break label; }");
        assert_eq!(d.len(), 1);
    }
}
