use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoCaseDeclarations;

impl Rule for NoCaseDeclarations {
    fn name(&self) -> &'static str {
        "no-case-declarations"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(node.kind(), "switch_case" | "switch_default") {
            return;
        }
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if matches!(
                child.kind(),
                "lexical_declaration" | "function_declaration" | "class_declaration"
            ) {
                ctx.report(
                    child.start_byte() as u32,
                    child.end_byte() as u32,
                    "Unexpected lexical declaration in case clause.",
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
            Box::new(NoCaseDeclarations),
            "switch(x) { case 1: { let a = 1; break; } }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoCaseDeclarations),
            "switch(x) { case 1: let a = 1; break; }",
        );
        assert_eq!(d.len(), 1);
    }
}
