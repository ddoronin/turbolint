use crate::diagnostic::Diagnostic;
use crate::disable_directives::filter_disabled;
use crate::line_index::LineIndex;
use crate::rule::Rule;
use crate::traverser::run_rules;

pub struct LintResult {
    pub diagnostics: Vec<Diagnostic>,
    pub line_index: LineIndex,
}

pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    pub fn lint(&self, source: &str) -> LintResult {
        let line_index = LineIndex::new(source);
        let tree = rslint_parser::parse(source);

        if tree.root_node().has_error() {
            return LintResult {
                diagnostics: Vec::new(),
                line_index,
            };
        }

        let root = tree.root_node();
        let diagnostics = run_rules(root, &self.rules, source);
        let diagnostics = filter_disabled(diagnostics, source, &line_index);
        LintResult {
            diagnostics,
            line_index,
        }
    }
}
