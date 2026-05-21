use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoArrayConstructor;

impl Rule for NoArrayConstructor {
    fn name(&self) -> &'static str {
        "no-array-constructor"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        let callee_name = match node.kind() {
            "new_expression" => node
                .child_by_field_name("constructor")
                .map(|c| ctx.node_text(&c).to_string()),
            "call_expression" => node
                .child_by_field_name("function")
                .filter(|f| f.kind() == "identifier")
                .map(|f| ctx.node_text(&f).to_string()),
            _ => None,
        };
        if let Some(name) = callee_name {
            if name == "Array" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "The array literal notation [] is preferable.",
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
        assert!(lint(Box::new(NoArrayConstructor), "var x = [1, 2, 3];").is_empty());
    }
    #[test]
    fn invalid_new() {
        let d = lint(Box::new(NoArrayConstructor), "var x = new Array();");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_call() {
        let d = lint(Box::new(NoArrayConstructor), "var x = Array(1, 2, 3);");
        assert_eq!(d.len(), 1);
    }
}
