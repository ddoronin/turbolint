use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct MaxLines;

const DEFAULT_MAX: usize = 300;

impl Rule for MaxLines {
    fn name(&self) -> &'static str {
        "max-lines"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "program" {
            return;
        }
        let line_count = ctx.source_text().lines().count();
        if line_count > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "File has too many lines ({}). Maximum allowed is {}.",
                    line_count, DEFAULT_MAX
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
        assert!(lint(Box::new(MaxLines), "var x = 1;\n").is_empty());
    }
}
