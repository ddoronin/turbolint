use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoPathConcat;

impl Rule for NoPathConcat {
    fn name(&self) -> &'static str {
        "no-path-concat"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "binary_expression" {
            return;
        }
        let op = match node.child_by_field_name("operator") {
            Some(o) => o,
            None => return,
        };
        if ctx.node_text(&op) != "+" {
            return;
        }
        let left = match node.child_by_field_name("left") {
            Some(l) => l,
            None => return,
        };
        let left_text = ctx.node_text(&left);
        if left_text == "__dirname" || left_text == "__filename" {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                "Use path.join() or path.resolve() instead of string concatenation.",
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
            Box::new(NoPathConcat),
            r#"var x = path.join(__dirname, "foo");"#
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoPathConcat), r#"var x = __dirname + "/foo";"#);
        assert_eq!(d.len(), 1);
    }
}
