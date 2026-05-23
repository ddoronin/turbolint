use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::{Fix, Severity, Span};
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUndefInit;

impl Rule for NoUndefInit {
    fn name(&self) -> &'static str {
        "no-undef-init"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "variable_declarator" {
            return;
        }
        if let Some(value) = node.child_by_field_name("value") {
            if value.kind() == "undefined" {
                let start = node.start_byte() as u32;
                let end = node.end_byte() as u32;
                // Find the name node to know where to cut from
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name_end = name_node.end_byte() as u32;
                    ctx.report_with_fix(
                        start,
                        end,
                        "It's not necessary to initialize to undefined.",
                        Fix {
                            range: Span { start: name_end, end: value.end_byte() as u32 },
                            text: String::new(),
                        },
                    );
                } else {
                    ctx.report(
                        start,
                        end,
                        "It's not necessary to initialize to undefined.",
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
    use turbolint_core::Linter;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUndefInit), "var x;").is_empty());
        assert!(lint(Box::new(NoUndefInit), "var x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUndefInit), "var x = undefined;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn autofix_removes_undefined_init() {
        let linter = Linter::new(vec![Box::new(NoUndefInit)]);
        let result = linter.lint_and_fix("var x = undefined;");
        assert!(result.fixed);
        assert_eq!(result.output, "var x;");
    }
    #[test]
    fn autofix_let() {
        let linter = Linter::new(vec![Box::new(NoUndefInit)]);
        let result = linter.lint_and_fix("let y = undefined;");
        assert!(result.fixed);
        assert_eq!(result.output, "let y;");
    }
}
