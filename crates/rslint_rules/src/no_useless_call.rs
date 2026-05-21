use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessCall;

impl Rule for NoUselessCall {
    fn name(&self) -> &'static str {
        "no-useless-call"
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
        let method = ctx.node_text(&prop);
        if method != "call" && method != "apply" {
            return;
        }
        let args = match node.child_by_field_name("arguments") {
            Some(a) => a,
            None => return,
        };
        let mut cursor = args.walk();
        let first_arg = args.named_children(&mut cursor).next();
        if let Some(arg) = first_arg {
            let text = ctx.node_text(&arg);
            if text == "null" || text == "undefined" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!("Unnecessary use of '.{}'.", method),
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
        assert!(lint(Box::new(NoUselessCall), "foo.call(obj, 1);").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUselessCall), "foo.call(null, 1);");
        assert_eq!(d.len(), 1);
    }
}
