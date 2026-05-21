use crate::diagnostic::Diagnostic;
use crate::disable_directives::filter_disabled;
use crate::fixer::apply_fixes;
use crate::line_index::LineIndex;
use crate::rule::Rule;
use crate::traverser::run_rules;

const MAX_AUTOFIX_PASSES: usize = 10;

pub struct LintResult {
    pub diagnostics: Vec<Diagnostic>,
    pub line_index: LineIndex,
}

pub struct FixResult {
    pub output: String,
    pub fixed: bool,
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

    pub fn lint_and_fix(&self, source: &str) -> FixResult {
        let mut current = source.to_string();
        let mut fixed = false;
        let mut prev_prev: Option<String> = None;

        for _ in 0..MAX_AUTOFIX_PASSES {
            let result = self.lint(&current);
            let apply = apply_fixes(&current, result.diagnostics);
            if !apply.fixed {
                break;
            }
            fixed = true;
            // Circular detection: if output matches 2 iterations ago, stop
            if let Some(ref pp) = prev_prev {
                if apply.output == *pp {
                    break;
                }
            }
            prev_prev = Some(current);
            current = apply.output;
        }

        // Final lint pass for accurate diagnostics
        let final_result = self.lint(&current);
        FixResult {
            output: current,
            fixed,
            diagnostics: final_result.diagnostics,
            line_index: final_result.line_index,
        }
    }
}
