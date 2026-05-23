use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::{Fix, Severity, Span};
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessRename;

impl Rule for NoUselessRename {
    fn name(&self) -> &'static str {
        "no-useless-rename"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        // import { a as a } or export { a as a }
        if node.kind() == "import_specifier" || node.kind() == "export_specifier" {
            let name = node.child_by_field_name("name");
            let alias = node.child_by_field_name("alias");
            if let (Some(n), Some(a)) = (name, alias) {
                let name_text = ctx.node_text(&n);
                if name_text == ctx.node_text(&a) {
                    let start = node.start_byte() as u32;
                    let end = node.end_byte() as u32;
                    // Fix: replace "x as x" with just "x"
                    ctx.report_with_fix(
                        start,
                        end,
                        format!("'{}' is uselessly renamed.", name_text),
                        Fix {
                            range: Span { start, end },
                            text: name_text.to_string(),
                        },
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
        assert!(lint(Box::new(NoUselessRename), "import { a } from 'b';").is_empty());
    }

    #[test]
    fn autofix_import() {
        let linter = Linter::new(vec![Box::new(NoUselessRename)]);
        let result = linter.lint_and_fix("import { x as x } from 'mod';");
        assert!(result.fixed);
        assert_eq!(result.output, "import { x } from 'mod';");
    }

    #[test]
    fn autofix_export() {
        let linter = Linter::new(vec![Box::new(NoUselessRename)]);
        let result = linter.lint_and_fix("export { foo as foo };");
        assert!(result.fixed);
        assert_eq!(result.output, "export { foo };");
    }
}
