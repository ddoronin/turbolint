use crate::helpers::unwrap_parens;
use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct UseIsnan;

impl Rule for UseIsnan {
    fn name(&self) -> &'static str {
        "use-isnan"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "binary_expression" => check_binary(node, ctx),
            "switch_statement" => check_switch(node, ctx),
            _ => {}
        }
    }
}

fn check_binary(node: &Node, ctx: &RuleContext) {
    let op = match node.child_by_field_name("operator") {
        Some(o) => o,
        None => return,
    };
    let op_text = ctx.node_text(&op);
    if !matches!(
        op_text,
        "==" | "===" | "!=" | "!==" | "<" | ">" | "<=" | ">="
    ) {
        return;
    }
    let left = node.child_by_field_name("left");
    let right = node.child_by_field_name("right");
    if left.is_some_and(|n| ctx.node_text(&n) == "NaN")
        || right.is_some_and(|n| ctx.node_text(&n) == "NaN")
    {
        ctx.report(
            node.start_byte() as u32,
            node.end_byte() as u32,
            "Use Number.isNaN() instead of comparison with NaN.",
        );
    }
}

fn check_switch(node: &Node, ctx: &RuleContext) {
    if let Some(value) = node.child_by_field_name("value") {
        let inner = unwrap_parens(&value);
        if ctx.node_text(&inner) == "NaN" {
            ctx.report(
                node.start_byte() as u32,
                value.end_byte() as u32,
                "'switch(NaN)' can never match a case clause. Use Number.isNaN() instead.",
            );
        }
    }
    if let Some(body) = node.child_by_field_name("body") {
        let mut cursor = body.walk();
        for child in body.named_children(&mut cursor) {
            if child.kind() == "switch_case" {
                if let Some(value) = child.child_by_field_name("value") {
                    if ctx.node_text(&value) == "NaN" {
                        ctx.report(
                            child.start_byte() as u32,
                            value.end_byte() as u32,
                            "'case NaN' can never match. Use Number.isNaN() instead.",
                        );
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
        assert!(lint(Box::new(UseIsnan), "Number.isNaN(x);").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(UseIsnan), "if (x === NaN) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_switch_nan() {
        let d = lint(Box::new(UseIsnan), "switch (NaN) { case 1: break; }");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_case_nan() {
        let d = lint(Box::new(UseIsnan), "switch (x) { case NaN: break; }");
        assert_eq!(d.len(), 1);
    }
}
