use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct MaxStatements;

const DEFAULT_MAX: usize = 10;

impl Rule for MaxStatements {
    fn name(&self) -> &'static str {
        "max-statements"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "function_declaration" | "function_expression" | "arrow_function" | "method_definition"
        ) {
            return;
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        if body.kind() != "statement_block" {
            return;
        }
        let count = body.named_child_count();
        if count > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "Function has too many statements ({}). Maximum allowed is {}.",
                    count, DEFAULT_MAX
                ),
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
        assert!(lint(Box::new(MaxStatements), "function foo() { var x = 1; }").is_empty());
    }
}
