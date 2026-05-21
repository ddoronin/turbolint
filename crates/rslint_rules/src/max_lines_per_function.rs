use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct MaxLinesPerFunction;

const DEFAULT_MAX: usize = 50;

impl Rule for MaxLinesPerFunction {
    fn name(&self) -> &'static str {
        "max-lines-per-function"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "function_declaration" | "function_expression" | "arrow_function"
        ) {
            return;
        }
        let lines = node.end_position().row - node.start_position().row + 1;
        if lines > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "Function has too many lines ({}). Maximum allowed is {}.",
                    lines, DEFAULT_MAX
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
        assert!(lint(
            Box::new(MaxLinesPerFunction),
            "function foo() { return 1; }"
        )
        .is_empty());
    }
}
