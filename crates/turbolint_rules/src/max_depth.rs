use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct MaxDepth;

const DEFAULT_MAX: usize = 4;

impl Rule for MaxDepth {
    fn name(&self) -> &'static str {
        "max-depth"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(
            node.kind(),
            "if_statement"
                | "for_statement"
                | "for_in_statement"
                | "while_statement"
                | "do_statement"
                | "switch_statement"
        ) {
            return;
        }
        let depth = nesting_depth(node);
        if depth > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!("Blocks are nested too deeply ({}).", depth),
            );
        }
    }
}

fn nesting_depth(node: &Node) -> usize {
    let mut depth = 1;
    let mut current = node.parent();
    while let Some(p) = current {
        if matches!(
            p.kind(),
            "if_statement"
                | "for_statement"
                | "for_in_statement"
                | "while_statement"
                | "do_statement"
                | "switch_statement"
        ) {
            depth += 1;
        }
        if matches!(
            p.kind(),
            "function_declaration" | "function_expression" | "arrow_function" | "method_definition"
        ) {
            break;
        }
        current = p.parent();
    }
    depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(
            Box::new(MaxDepth),
            "if (a) { if (b) { if (c) { if (d) {} } } }"
        )
        .is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(MaxDepth),
            "if (a) { if (b) { if (c) { if (d) { if (e) {} } } } }",
        );
        assert!(d.len() >= 1);
    }
}
