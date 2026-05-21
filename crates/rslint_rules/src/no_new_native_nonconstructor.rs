use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoNewNativeNonconstructor;

impl Rule for NoNewNativeNonconstructor {
    fn name(&self) -> &'static str {
        "no-new-native-nonconstructor"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "new_expression" {
            return;
        }
        if let Some(constructor) = node.child_by_field_name("constructor") {
            let name = ctx.node_text(&constructor);
            if name == "Symbol" || name == "BigInt" {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!("{} is not a constructor.", name),
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
            Box::new(NoNewNativeNonconstructor),
            "var s = Symbol('foo');"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoNewNativeNonconstructor),
            "var s = new Symbol('foo');",
        );
        assert_eq!(d.len(), 1);
    }
}
