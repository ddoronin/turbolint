use std::cell::RefCell;

use tree_sitter::Node;

use crate::context::RuleContext;
use crate::diagnostic::Diagnostic;
use crate::rule::Rule;

pub fn run_rules(root: Node, rules: &[Box<dyn Rule>], source_text: &str) -> Vec<Diagnostic> {
    let diagnostics = RefCell::new(Vec::new());
    let contexts: Vec<RuleContext> = rules
        .iter()
        .map(|rule| {
            RuleContext::new(
                source_text,
                rule.name(),
                rule.default_severity(),
                &diagnostics,
            )
        })
        .collect();
    traverse(root, rules, &contexts);
    diagnostics.into_inner()
}

fn traverse(node: Node, rules: &[Box<dyn Rule>], contexts: &[RuleContext]) {
    for (rule, ctx) in rules.iter().zip(contexts.iter()) {
        rule.on_node(&node, ctx);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        traverse(child, rules, contexts);
    }
}
