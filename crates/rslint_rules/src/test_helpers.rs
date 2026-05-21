use rslint_core::{Diagnostic, Linter, Rule};

/// Lint the given source code with a single rule and return diagnostics.
pub fn lint(rule: Box<dyn Rule>, code: &str) -> Vec<Diagnostic> {
    let linter = Linter::new(vec![rule]);
    linter.lint(code).diagnostics
}

/// Assert that linting produces no diagnostics.
pub fn assert_no_lint(rule: Box<dyn Rule>, code: &str) {
    let diagnostics = lint(rule, code);
    assert!(
        diagnostics.is_empty(),
        "Expected no diagnostics, got {}: {:?}",
        diagnostics.len(),
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}

/// Assert that linting produces exactly `expected` diagnostics.
pub fn assert_lint(rule: Box<dyn Rule>, code: &str, expected: usize) {
    let diagnostics = lint(rule, code);
    assert_eq!(
        diagnostics.len(),
        expected,
        "Expected {} diagnostics, got {}: {:?}",
        expected,
        diagnostics.len(),
        diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>()
    );
}
