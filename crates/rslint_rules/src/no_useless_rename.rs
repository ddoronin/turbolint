use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
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
        // import { a as a } or export { a as a } or const { a: a } = obj
        if node.kind() == "import_specifier" || node.kind() == "export_specifier" {
            let name = node.child_by_field_name("name");
            let alias = node.child_by_field_name("alias");
            if let (Some(n), Some(a)) = (name, alias) {
                if ctx.node_text(&n) == ctx.node_text(&a) {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        format!("'{}' is uselessly renamed.", ctx.node_text(&n)),
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

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUselessRename), "import { a } from 'b';").is_empty());
    }
}
