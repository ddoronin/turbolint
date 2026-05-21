use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoExtraBind;

impl Rule for NoExtraBind {
    fn name(&self) -> &'static str {
        "no-extra-bind"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        let func = match node.child_by_field_name("function") {
            Some(f) => f,
            None => return,
        };
        if func.kind() != "member_expression" {
            return;
        }
        let prop = match func.child_by_field_name("property") {
            Some(p) => p,
            None => return,
        };
        if ctx.node_text(&prop) != "bind" {
            return;
        }
        let obj = match func.child_by_field_name("object") {
            Some(o) => o,
            None => return,
        };
        // Check if the object is an arrow function (which ignores .bind())
        // It may be wrapped in parentheses
        let inner = if obj.kind() == "parenthesized_expression" {
            obj.named_child(0)
        } else {
            Some(obj)
        };
        let is_arrow = inner.is_some_and(|n| n.kind() == "arrow_function");
        if is_arrow {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Unnecessary use of .bind() with arrow function.",
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
            Box::new(NoExtraBind),
            "var f = function() { this.x; }.bind(obj);"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoExtraBind), "var f = (() => {}).bind(obj);");
        assert_eq!(d.len(), 1);
    }
}
