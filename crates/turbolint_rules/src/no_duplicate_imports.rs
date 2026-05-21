use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct NoDuplicateImports;
impl Rule for NoDuplicateImports {
    fn name(&self) -> &'static str {
        "no-duplicate-imports"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "program" {
            return;
        }
        let mut sources: Vec<String> = Vec::new();
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == "import_statement" {
                if let Some(source) = child.child_by_field_name("source") {
                    let text = ctx.node_text(&source).to_string();
                    if sources.contains(&text) {
                        ctx.report(
                            child.start_byte() as u32,
                            child.end_byte() as u32,
                            format!("'{}' imported multiple times.", text),
                        );
                    } else {
                        sources.push(text);
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
        assert!(lint(Box::new(NoDuplicateImports), "import { a } from 'b';").is_empty());
    }
}
