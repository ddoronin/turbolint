use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct DefaultParamLast;

impl Rule for DefaultParamLast {
    fn name(&self) -> &'static str {
        "default-param-last"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "formal_parameters" {
            return;
        }
        let mut cursor = node.walk();
        let params: Vec<_> = node.named_children(&mut cursor).collect();
        let mut seen_non_default = false;
        // Iterate in reverse: once we see a non-default param, any default param is an error
        for param in params.iter().rev() {
            if param.kind() == "assignment_pattern" {
                if seen_non_default {
                    ctx.report(
                        param.start_byte() as u32,
                        param.end_byte() as u32,
                        "Default parameters should be last.",
                    );
                }
            } else {
                seen_non_default = true;
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
        assert!(lint(Box::new(DefaultParamLast), "function foo(a, b = 1) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(DefaultParamLast), "function foo(a = 1, b) {}");
        assert_eq!(d.len(), 1);
    }
}
