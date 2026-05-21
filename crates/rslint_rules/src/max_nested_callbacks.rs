use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct MaxNestedCallbacks;

const DEFAULT_MAX: usize = 10;

impl Rule for MaxNestedCallbacks {
    fn name(&self) -> &'static str {
        "max-nested-callbacks"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if !matches!(node.kind(), "function_expression" | "arrow_function") {
            return;
        }
        // Only check if this is a callback (argument to a call)
        if let Some(parent) = node.parent() {
            if parent.kind() != "arguments" {
                return;
            }
        }
        let depth = callback_depth(node);
        if depth > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!("Too many nested callbacks ({}).", depth),
            );
        }
    }
}

fn callback_depth(node: &Node) -> usize {
    let mut depth = 1;
    let mut current = node.parent();
    while let Some(p) = current {
        if matches!(p.kind(), "function_expression" | "arrow_function") {
            if let Some(pp) = p.parent() {
                if pp.kind() == "arguments" {
                    depth += 1;
                }
            }
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
        assert!(lint(Box::new(MaxNestedCallbacks), "foo(function() {});").is_empty());
    }
}
