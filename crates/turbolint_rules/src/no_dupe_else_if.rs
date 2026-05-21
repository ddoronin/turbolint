use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoDupeElseIf;

impl Rule for NoDupeElseIf {
    fn name(&self) -> &'static str {
        "no-dupe-else-if"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "if_statement" {
            return;
        }
        // Only process the top-level if (skip if inside else_clause)
        if let Some(parent) = node.parent() {
            if parent.kind() == "else_clause" {
                return;
            }
        }
        let mut conditions: Vec<(String, u32, u32)> = Vec::new();
        collect_conditions(node, ctx, &mut conditions);
        for i in 1..conditions.len() {
            for j in 0..i {
                if conditions[i].0 == conditions[j].0 {
                    ctx.report(
                        conditions[i].1,
                        conditions[i].2,
                        "This branch can never execute. Its condition is a duplicate of a previous condition in the if-else-if chain.",
                    );
                    break;
                }
            }
        }
    }
}

fn collect_conditions(node: &Node, ctx: &RuleContext, out: &mut Vec<(String, u32, u32)>) {
    if node.kind() != "if_statement" {
        return;
    }
    if let Some(cond) = node.child_by_field_name("condition") {
        out.push((
            ctx.node_text(&cond).to_string(),
            cond.start_byte() as u32,
            cond.end_byte() as u32,
        ));
    }
    // In tree-sitter: if_statement has an "alternative" field which is an else_clause
    // else_clause contains either a statement_block or another if_statement
    if let Some(alt) = node.child_by_field_name("alternative") {
        // alt is the else_clause node
        let mut cursor = alt.walk();
        for child in alt.named_children(&mut cursor) {
            if child.kind() == "if_statement" {
                collect_conditions(&child, ctx, out);
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
            Box::new(NoDupeElseIf),
            "if (a) {} else if (b) {} else if (c) {}"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoDupeElseIf),
            "if (a) {} else if (b) {} else if (a) {}",
        );
        assert_eq!(d.len(), 1);
    }
}
