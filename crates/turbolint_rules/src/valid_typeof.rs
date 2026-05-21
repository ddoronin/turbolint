use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct ValidTypeof;
const VALID_TYPES: &[&str] = &[
    "\"undefined\"",
    "\"object\"",
    "\"boolean\"",
    "\"number\"",
    "\"string\"",
    "\"function\"",
    "\"symbol\"",
    "\"bigint\"",
    "'undefined'",
    "'object'",
    "'boolean'",
    "'number'",
    "'string'",
    "'function'",
    "'symbol'",
    "'bigint'",
];
impl Rule for ValidTypeof {
    fn name(&self) -> &'static str {
        "valid-typeof"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "binary_expression" {
            return;
        }
        let op = match node.child_by_field_name("operator") {
            Some(o) => o,
            None => return,
        };
        if !matches!(ctx.node_text(&op), "==" | "===" | "!=" | "!==") {
            return;
        }
        let left = node.child_by_field_name("left");
        let right = node.child_by_field_name("right");
        let (_typeof_node, value_node) = if left
            .as_ref()
            .is_some_and(|n| n.kind() == "typeof_expression")
        {
            (left, right)
        } else if right
            .as_ref()
            .is_some_and(|n| n.kind() == "typeof_expression")
        {
            (right, left)
        } else {
            return;
        };
        if let Some(val) = value_node {
            if val.kind() == "string" {
                let val_text = ctx.node_text(&val);
                if !VALID_TYPES.contains(&val_text) {
                    ctx.report(
                        val.start_byte() as u32,
                        val.end_byte() as u32,
                        format!("Invalid typeof comparison value: {}.", val_text),
                    );
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
        assert!(lint(Box::new(ValidTypeof), r#"typeof x === "string""#).is_empty());
    }
}
