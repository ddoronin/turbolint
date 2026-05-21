use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct MaxParams;

const DEFAULT_MAX: usize = 3;

impl Rule for MaxParams {
    fn name(&self) -> &'static str {
        "max-params"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "formal_parameters" {
            return;
        }
        let count = node.named_child_count();
        if count > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "Function has too many parameters ({}). Maximum allowed is {}.",
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
        assert!(lint(Box::new(MaxParams), "function foo(a, b, c) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(MaxParams), "function foo(a, b, c, d) {}");
        assert_eq!(d.len(), 1);
    }
}
