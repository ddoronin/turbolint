use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoNewWrappers;

const WRAPPERS: &[&str] = &["String", "Number", "Boolean"];

impl Rule for NoNewWrappers {
    fn name(&self) -> &'static str {
        "no-new-wrappers"
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
            if WRAPPERS.contains(&name) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    format!("Do not use {} as a constructor.", name),
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
        assert!(lint(Box::new(NoNewWrappers), "var x = String(42);").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoNewWrappers), "var x = new String('hello');");
        assert_eq!(d.len(), 1);
    }
}
