use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct PreferNumericLiterals;
impl Rule for PreferNumericLiterals {
    fn name(&self) -> &'static str {
        "prefer-numeric-literals"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "call_expression" {
            return;
        }
        if let Some(func) = node.child_by_field_name("function") {
            let name = ctx.node_text(&func);
            if name == "parseInt" || name == "Number.parseInt" {
                if let Some(args) = node.child_by_field_name("arguments") {
                    if args.named_child_count() >= 2 {
                        if let Some(radix) = args.named_child(1) {
                            let r = ctx.node_text(&radix);
                            if matches!(r, "2" | "8" | "16") {
                                ctx.report(
                                    node.start_byte() as u32,
                                    node.end_byte() as u32,
                                    "Use binary/octal/hex literals instead of parseInt().",
                                );
                            }
                        }
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
        assert!(lint(Box::new(PreferNumericLiterals), "var x = 0xFF;").is_empty());
    }
}
