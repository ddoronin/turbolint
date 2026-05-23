use crate::helpers::is_terminal;
use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUnreachable;

impl Rule for NoUnreachable {
    fn name(&self) -> &'static str {
        "no-unreachable"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "statement_block" | "program" => check_statements(node, ctx),
            "switch_case" | "switch_default" => check_case_body(node, ctx),
            _ => {}
        }
    }
}

fn check_statements(node: &Node, ctx: &RuleContext) {
    let mut cursor = node.walk();
    let mut found_terminal = false;
    for child in node.named_children(&mut cursor) {
        if found_terminal {
            ctx.report(
                child.start_byte() as u32,
                child.end_byte() as u32,
                "Unreachable code.",
            );
            continue;
        }
        if is_terminal(&child) {
            found_terminal = true;
        }
    }
}

fn check_case_body(node: &Node, ctx: &RuleContext) {
    let mut cursor = node.walk();
    let skip = if node.kind() == "switch_case" { 1 } else { 0 };
    let mut found_terminal = false;
    for child in node.named_children(&mut cursor).skip(skip) {
        if found_terminal {
            ctx.report(
                child.start_byte() as u32,
                child.end_byte() as u32,
                "Unreachable code.",
            );
            continue;
        }
        if is_terminal(&child) {
            found_terminal = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUnreachable), "function f() { return 1; }").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(
            Box::new(NoUnreachable),
            "function f() { return 1; var x = 2; }",
        );
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_multiple_unreachable() {
        let d = lint(
            Box::new(NoUnreachable),
            "function f() { return 1; var x = 2; var y = 3; }",
        );
        assert_eq!(d.len(), 2);
    }
    #[test]
    fn invalid_switch_case() {
        let d = lint(
            Box::new(NoUnreachable),
            "switch(x) { case 1: return; var a = 1; }",
        );
        assert_eq!(d.len(), 1);
    }
}
