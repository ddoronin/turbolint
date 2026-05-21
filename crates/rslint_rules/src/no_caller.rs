use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoCaller;

impl Rule for NoCaller {
    fn name(&self) -> &'static str {
        "no-caller"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "member_expression" {
            return;
        }
        let obj = match node.child_by_field_name("object") {
            Some(o) => o,
            None => return,
        };
        let prop = match node.child_by_field_name("property") {
            Some(p) => p,
            None => return,
        };
        if ctx.node_text(&obj) == "arguments" {
            let prop_text = ctx.node_text(&prop);
            if prop_text == "callee" || prop_text == "caller" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!("'arguments.{}' is deprecated.", prop_text),
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
        assert!(lint(Box::new(NoCaller), "var x = foo.callee;").is_empty());
    }
    #[test]
    fn invalid_callee() {
        let d = lint(Box::new(NoCaller), "var x = arguments.callee;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_caller() {
        let d = lint(Box::new(NoCaller), "var x = arguments.caller;");
        assert_eq!(d.len(), 1);
    }
}
