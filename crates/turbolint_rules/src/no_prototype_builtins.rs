use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoPrototypeBuiltins;

const BUILTINS: &[&str] = &["hasOwnProperty", "isPrototypeOf", "propertyIsEnumerable"];

impl Rule for NoPrototypeBuiltins {
    fn name(&self) -> &'static str {
        "no-prototype-builtins"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
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
        if let Some(prop) = func.child_by_field_name("property") {
            let prop_text = ctx.node_text(&prop);
            if BUILTINS.contains(&prop_text) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!(
                        "Do not access Object.prototype method '{}' from target object.",
                        prop_text
                    ),
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
        assert!(lint(
            Box::new(NoPrototypeBuiltins),
            "Object.prototype.hasOwnProperty.call(obj, 'foo');"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoPrototypeBuiltins), "obj.hasOwnProperty('foo');");
        assert_eq!(d.len(), 1);
    }
}
