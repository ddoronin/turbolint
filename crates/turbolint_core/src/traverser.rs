use std::cell::RefCell;

use tree_sitter::Node;

use crate::context::RuleContext;
use crate::diagnostic::{Diagnostic, Severity};
use crate::rule::Rule;

pub fn run_rules(root: Node, rules: &[Box<dyn Rule>], source_text: &str) -> Vec<Diagnostic> {
    let severities: Vec<Severity> = rules.iter().map(|r| r.default_severity()).collect();
    run_rules_with_severities(root, rules, &severities, source_text)
}

pub fn run_rules_with_severities(
    root: Node,
    rules: &[Box<dyn Rule>],
    severities: &[Severity],
    source_text: &str,
) -> Vec<Diagnostic> {
    let diagnostics = RefCell::new(Vec::new());
    let contexts: Vec<RuleContext> = rules
        .iter()
        .zip(severities.iter())
        .map(|(rule, &severity)| {
            RuleContext::new(source_text, rule.name(), severity, &diagnostics)
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
