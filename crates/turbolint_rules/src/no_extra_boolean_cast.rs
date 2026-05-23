use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoExtraBooleanCast;

impl Rule for NoExtraBooleanCast {
    fn name(&self) -> &'static str {
        "no-extra-boolean-cast"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "unary_expression" => check_double_negation(node, ctx),
            "call_expression" => check_boolean_call(node, ctx),
            _ => {}
        }
    }
}

fn check_double_negation(node: &Node, ctx: &RuleContext) {
    let op = match node.child_by_field_name("operator") {
        Some(o) => o,
        None => return,
    };
    if ctx.node_text(&op) != "!" {
        return;
    }
    let arg = match node.child_by_field_name("argument") {
        Some(a) => a,
        None => return,
    };
    if arg.kind() == "unary_expression" {
        if let Some(inner_op) = arg.child_by_field_name("operator") {
            if ctx.node_text(&inner_op) == "!" {
                // !! found — check if in a boolean context
                if is_in_boolean_context(node) {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "Redundant double negation.",
                    );
                }
            }
        }
    }
}

fn check_boolean_call(node: &Node, ctx: &RuleContext) {
    if let Some(func) = node.child_by_field_name("function") {
        if ctx.node_text(&func) == "Boolean" {
            if is_in_boolean_context(node) {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Redundant Boolean call in boolean context.",
                );
            }
        }
    }
}

fn is_in_boolean_context(node: &Node) -> bool {
    let mut current = *node;
    loop {
        let parent = match current.parent() {
            Some(p) => p,
            None => return false,
        };
        match parent.kind() {
            "if_statement" | "while_statement" | "do_statement" | "ternary_expression" => {
                return true;
            }
            "for_statement" => {
                // Only the condition part is boolean context
                if let Some(cond) = parent.child_by_field_name("condition") {
                    return contains_node(&cond, &current);
                }
                return false;
            }
            "unary_expression" => {
                // ! is a boolean context (so !!!x gets flagged)
                if let Some(op) = parent.child_by_field_name("operator") {
                    if op.kind() == "!" {
                        return true;
                    }
                }
                return false;
            }
            "parenthesized_expression" => {
                current = parent;
                continue;
            }
            _ => return false,
        }
    }
}

fn contains_node(haystack: &Node, needle: &Node) -> bool {
    needle.start_byte() >= haystack.start_byte() && needle.end_byte() <= haystack.end_byte()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoExtraBooleanCast), "if (foo) {}").is_empty());
        assert!(lint(Box::new(NoExtraBooleanCast), "var x = !!foo;").is_empty());
        assert!(lint(Box::new(NoExtraBooleanCast), "var x = Boolean(foo);").is_empty());
    }
    #[test]
    fn invalid_double_negation_in_if() {
        let d = lint(Box::new(NoExtraBooleanCast), "if (!!foo) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_boolean_call_in_if() {
        let d = lint(Box::new(NoExtraBooleanCast), "if (Boolean(foo)) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_triple_negation() {
        let d = lint(Box::new(NoExtraBooleanCast), "var x = !!!foo;");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_for_condition() {
        let d = lint(Box::new(NoExtraBooleanCast), "for (;!!foo;) {}");
        assert_eq!(d.len(), 1);
    }
}
