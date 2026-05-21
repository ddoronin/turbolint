use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct PreferPromiseRejectErrors;
impl Rule for PreferPromiseRejectErrors {
    fn name(&self) -> &'static str {
        "prefer-promise-reject-errors"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        if let Some(func) = node.child_by_field_name("function") {
            if func.kind() == "member_expression" {
                let obj = func.child_by_field_name("object");
                let prop = func.child_by_field_name("property");
                if let (Some(o), Some(p)) = (obj, prop) {
                    if ctx.node_text(&o) == "Promise" && ctx.node_text(&p) == "reject" {
                        if let Some(args) = node.child_by_field_name("arguments") {
                            if let Some(first) = args.named_child(0) {
                                if matches!(first.kind(), "string" | "number") {
                                    ctx.report(
                                        node.start_byte() as u32,
                                        node.end_byte() as u32,
                                        "Expected the Promise rejection reason to be an Error.",
                                    );
                                }
                            }
                        }
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
            Box::new(PreferPromiseRejectErrors),
            "Promise.reject(new Error('msg'));"
        )
        .is_empty());
    }
}
