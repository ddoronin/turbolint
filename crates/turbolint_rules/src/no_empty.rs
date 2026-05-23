use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoEmpty;

impl Rule for NoEmpty {
    fn name(&self) -> &'static str {
        "no-empty"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "statement_block" => {
                if node.named_child_count() == 0 {
                    // Allow empty function bodies
                    if let Some(parent) = node.parent() {
                        if matches!(
                            parent.kind(),
                            "function_declaration"
                                | "function_expression"
                                | "arrow_function"
                                | "method_definition"
                                | "generator_function"
                                | "generator_function_declaration"
                        ) {
                            return;
                        }
                    }
                    // Allow blocks that contain comments
                    let source = ctx.source_text();
                    let block_text = &source[node.start_byte()..node.end_byte()];
                    if block_text.contains("//") || block_text.contains("/*") {
                        return;
                    }
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Empty block statement.",
                    );
                }
            }
            "switch_statement" => {
                // Check for empty switch body (no cases)
                if let Some(body) = node.child_by_field_name("body") {
                    if body.named_child_count() == 0 {
                        // Allow if contains comments
                        let source = ctx.source_text();
                        let body_text = &source[body.start_byte()..body.end_byte()];
                        if body_text.contains("//") || body_text.contains("/*") {
                            return;
                        }
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Empty switch statement.",
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoEmpty), "if (foo) { bar(); }").is_empty());
        assert!(lint(Box::new(NoEmpty), "function foo() {}").is_empty());
        assert!(lint(Box::new(NoEmpty), "if (foo) { /* comment */ }").is_empty());
        assert!(lint(Box::new(NoEmpty), "if (foo) { // comment\n}").is_empty());
    }
    #[test]
    fn valid_switch_with_cases() {
        assert!(lint(Box::new(NoEmpty), "switch (x) { case 1: break; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoEmpty), "if (foo) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_empty_catch() {
        let d = lint(Box::new(NoEmpty), "try { foo(); } catch (e) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_empty_switch() {
        let d = lint(Box::new(NoEmpty), "switch (x) {}");
        assert_eq!(d.len(), 1);
    }
}
