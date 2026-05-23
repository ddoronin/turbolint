use tree_sitter::Node;

use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::{Fix, Severity, Span};
use turbolint_core::Rule;

pub struct NoDebugger;

impl Rule for NoDebugger {
    fn name(&self) -> &'static str {
        "no-debugger"
    }

    fn default_severity(&self) -> Severity {
        Severity::Error
    }

    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "debugger_statement" {
            let start = node.start_byte() as u32;
            let end = node.end_byte() as u32;
            ctx.report_with_fix(
                start,
                end,
                "Unexpected 'debugger' statement.",
                Fix {
                    range: Span { start, end },
                    text: String::new(),
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use turbolint_core::Linter;

    fn lint(source: &str) -> Vec<turbolint_core::Diagnostic> {
        let linter = Linter::new(vec![Box::new(NoDebugger)]);
        linter.lint(source).diagnostics
    }

    #[test]
    fn reports_debugger() {
        let diagnostics = lint("debugger;");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "no-debugger");
        assert_eq!(diagnostics[0].message, "Unexpected 'debugger' statement.");
        assert_eq!(diagnostics[0].span.start, 0);
    }

    #[test]
    fn no_error_on_clean_code() {
        let diagnostics = lint("var x = 1;");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn autofix_removes_debugger() {
        let linter = Linter::new(vec![Box::new(NoDebugger)]);
        let result = linter.lint_and_fix("debugger;");
        assert!(result.fixed);
        assert_eq!(result.output, "");
    }

    #[test]
    fn autofix_preserves_surrounding_code() {
        let linter = Linter::new(vec![Box::new(NoDebugger)]);
        let result = linter.lint_and_fix("var x = 1;\ndebugger;\nvar y = 2;");
        assert!(result.fixed);
        assert_eq!(result.output, "var x = 1;\n\nvar y = 2;");
    }

    // ================================================================
    // eslint-disable-line — line comments (//)
    // ================================================================

    #[test]
    fn disable_line_suppresses_all_rules() {
        let diagnostics = lint("debugger; // eslint-disable-line\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_line_with_matching_rule() {
        let diagnostics = lint("debugger; // eslint-disable-line no-debugger\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_line_with_non_matching_rule_still_reports() {
        let diagnostics = lint("debugger; // eslint-disable-line no-console\n");
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn disable_line_with_multiple_rules_including_match() {
        let diagnostics = lint("debugger; // eslint-disable-line no-console, no-debugger\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_line_with_multiple_rules_no_match() {
        let diagnostics = lint("debugger; // eslint-disable-line no-console, no-alert\n");
        assert_eq!(diagnostics.len(), 1);
    }

    // ================================================================
    // eslint-disable-line — block comments (/* */)
    // ================================================================

    #[test]
    fn disable_line_block_comment() {
        let diagnostics = lint("debugger; /* eslint-disable-line */\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_line_block_comment_with_rule() {
        let diagnostics = lint("debugger; /* eslint-disable-line no-debugger */\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_line_block_comment_multiple_rules() {
        let diagnostics = lint("debugger; /* eslint-disable-line no-console, no-debugger */\n");
        assert!(diagnostics.is_empty());
    }

    // ================================================================
    // eslint-disable-next-line — line comments (//)
    // ================================================================

    #[test]
    fn disable_next_line_suppresses_all_rules() {
        let diagnostics = lint("// eslint-disable-next-line\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_with_matching_rule() {
        let diagnostics = lint("// eslint-disable-next-line no-debugger\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_with_non_matching_rule_still_reports() {
        let diagnostics = lint("// eslint-disable-next-line no-console\ndebugger;\n");
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn disable_next_line_with_multiple_rules_including_match() {
        let diagnostics = lint("// eslint-disable-next-line no-console, no-debugger\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_with_multiple_rules_no_match() {
        let diagnostics = lint("// eslint-disable-next-line no-console, no-alert\ndebugger;\n");
        assert_eq!(diagnostics.len(), 1);
    }

    // ================================================================
    // eslint-disable-next-line — block comments (/* */)
    // ================================================================

    #[test]
    fn disable_next_line_block_comment() {
        let diagnostics = lint("/* eslint-disable-next-line */\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_block_comment_with_rule() {
        let diagnostics = lint("/* eslint-disable-next-line no-debugger */\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_block_comment_multiple_rules() {
        let diagnostics =
            lint("/* eslint-disable-next-line no-console, no-debugger */\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    // ================================================================
    // /* eslint-disable */ block directives
    // ================================================================

    #[test]
    fn block_disable_all_suppresses_everything() {
        let src = "/* eslint-disable */\ndebugger;\ndebugger;\n";
        let diagnostics = lint(src);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn block_disable_specific_rule() {
        let src = "/* eslint-disable no-debugger */\ndebugger;\n";
        let diagnostics = lint(src);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn block_disable_non_matching_rule_still_reports() {
        let src = "/* eslint-disable no-console */\ndebugger;\n";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn block_disable_multiple_rules_with_match() {
        let src = "/* eslint-disable no-console, no-debugger */\ndebugger;\n";
        let diagnostics = lint(src);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn block_disable_at_top_of_file_suppresses_entire_file() {
        let src = "\
/* eslint-disable */
var x = 1;
debugger;
function foo() {
  debugger;
}
debugger;
";
        let diagnostics = lint(src);
        assert!(diagnostics.is_empty());
    }

    // ================================================================
    // /* eslint-enable */ re-enabling
    // ================================================================

    #[test]
    fn block_enable_re_enables_all() {
        let src = "\
/* eslint-disable */
debugger;
/* eslint-enable */
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(
            diagnostics.len(),
            1,
            "line 4 debugger should report after enable"
        );
    }

    #[test]
    fn block_enable_specific_rule() {
        let src = "\
/* eslint-disable no-debugger */
debugger;
/* eslint-enable no-debugger */
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn block_enable_non_matching_rule_keeps_disabled() {
        // Disable no-debugger, then enable no-console — no-debugger stays disabled
        let src = "\
/* eslint-disable no-debugger */
debugger;
/* eslint-enable no-console */
debugger;
";
        let diagnostics = lint(src);
        assert!(
            diagnostics.is_empty(),
            "no-debugger should still be disabled"
        );
    }

    #[test]
    fn block_enable_all_overrides_specific_disable() {
        // Disable specific rule, then enable all — should re-enable
        let src = "\
/* eslint-disable no-debugger */
debugger;
/* eslint-enable */
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(
            diagnostics.len(),
            1,
            "enable-all should re-enable no-debugger"
        );
    }

    #[test]
    fn block_disable_enable_disable_sandwich() {
        let src = "\
/* eslint-disable */
debugger;
/* eslint-enable */
debugger;
/* eslint-disable */
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1, "only line 4 should report");
    }

    #[test]
    fn block_disable_does_not_affect_lines_before() {
        let src = "\
debugger;
/* eslint-disable */
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1, "line 1 should still report");
    }

    // ================================================================
    // -- description / justification syntax
    // ================================================================

    #[test]
    fn disable_line_with_description() {
        let diagnostics = lint("debugger; // eslint-disable-line no-debugger -- needed for dev\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_with_description() {
        let diagnostics =
            lint("// eslint-disable-next-line no-debugger -- temporarily needed\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn block_disable_with_description() {
        let diagnostics = lint("/* eslint-disable no-debugger -- legacy code */\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_line_all_rules_with_description() {
        let diagnostics = lint("debugger; // eslint-disable-line -- suppress everything\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_all_rules_with_description() {
        let diagnostics = lint("// eslint-disable-next-line -- suppress everything\ndebugger;\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn description_with_long_separator() {
        let diagnostics =
            lint("debugger; // eslint-disable-line no-debugger -------- long reason here\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn multiple_rules_with_description() {
        let diagnostics =
            lint("debugger; // eslint-disable-line no-console, no-debugger -- reason\n");
        assert!(diagnostics.is_empty());
    }

    // ================================================================
    // Scoping: directives only affect their targeted lines
    // ================================================================

    #[test]
    fn disable_next_line_does_not_affect_other_lines() {
        let src = "// eslint-disable-next-line\ndebugger;\ndebugger;\n";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn disable_line_does_not_affect_other_lines() {
        let src = "debugger; // eslint-disable-line\ndebugger;\n";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn multiple_line_directives_combined() {
        let src = "\
debugger; // eslint-disable-line no-debugger
// eslint-disable-next-line
debugger;
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1, "only line 4 should report");
    }

    // ================================================================
    // Mixed: line directives + block directives
    // ================================================================

    #[test]
    fn line_directive_inside_block_disable() {
        // Block disables everything, line directive is redundant but shouldn't break
        let src = "\
/* eslint-disable */
debugger; // eslint-disable-line
debugger;
";
        let diagnostics = lint(src);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn line_directive_overrides_after_block_enable() {
        let src = "\
/* eslint-disable */
debugger;
/* eslint-enable */
debugger; // eslint-disable-line
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(diagnostics.len(), 1, "only line 5 should report");
    }

    #[test]
    fn next_line_directive_with_block_disable_for_different_rule() {
        let src = "\
/* eslint-disable no-console */
// eslint-disable-next-line no-debugger
debugger;
debugger;
";
        let diagnostics = lint(src);
        assert_eq!(
            diagnostics.len(),
            1,
            "line 4 should report (not covered by next-line)"
        );
    }

    // ================================================================
    // Edge cases
    // ================================================================

    #[test]
    fn no_false_positive_on_eslint_disable_in_string() {
        // The string contains "eslint-disable-line" but it's not a comment
        let diagnostics = lint("var s = '// eslint-disable-line';\ndebugger;\n");
        // The line-based parser will see "// eslint-disable-line" inside the string
        // and may suppress. This is a known limitation of line-based parsing
        // (ESLint uses AST comments, we use text scanning). We accept this for now.
        // The important thing is debugger on line 2 still reports.
        assert!(diagnostics.len() >= 1, "line 2 debugger should report");
    }

    #[test]
    fn disable_line_at_end_of_file_no_trailing_newline() {
        let diagnostics = lint("debugger; // eslint-disable-line");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_next_line_at_end_of_file_no_target_line() {
        // disable-next-line with nothing after it — should not crash
        let diagnostics = lint("// eslint-disable-next-line");
        assert!(diagnostics.is_empty(), "no diagnostics to suppress");
    }

    #[test]
    fn block_disable_with_no_enable_suppresses_to_eof() {
        let src = "\
var x = 1;
/* eslint-disable no-debugger */
debugger;
debugger;
debugger;
";
        let diagnostics = lint(src);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn empty_source() {
        let diagnostics = lint("");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn only_comments_no_code() {
        let diagnostics = lint("// eslint-disable-next-line\n// just a comment\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn disable_line_with_extra_whitespace() {
        let diagnostics = lint("debugger; //   eslint-disable-line   no-debugger  \n");
        // Extra spaces between // and eslint-disable-line: ESLint requires
        // exactly `// eslint-disable-line` (one space). With extra spaces,
        // this should NOT suppress (ESLint behavior).
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn block_disable_and_enable_on_same_line_as_code() {
        // Both disable and enable on the same line. ESLint uses column-level
        // granularity, but turbolint uses line-level. With line-level, the enable
        // is processed after disable (source order), so line 1 ends up enabled.
        // Line 2 also reports because the enable is on line 1.
        let src = "/* eslint-disable */ debugger; /* eslint-enable */\ndebugger;\n";
        let diagnostics = lint(src);
        assert_eq!(
            diagnostics.len(),
            2,
            "both debuggers report (line-level granularity)"
        );
    }

    #[test]
    fn plugin_style_rule_name() {
        // ESLint supports plugin rules like "example/rule-name"
        let diagnostics = lint("debugger; // eslint-disable-line example/no-debugger\n");
        assert_eq!(
            diagnostics.len(),
            1,
            "plugin rule name should not match 'no-debugger'"
        );
    }

    #[test]
    fn three_rules_in_list() {
        let diagnostics =
            lint("debugger; // eslint-disable-line no-alert, no-debugger, no-console\n");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn block_disable_multiple_enable_specific() {
        // Disable all, then enable only no-console — no-debugger stays disabled
        let src = "\
/* eslint-disable */
debugger;
/* eslint-enable no-console */
debugger;
";
        let diagnostics = lint(src);
        assert!(
            diagnostics.is_empty(),
            "no-debugger should still be disabled after enabling only no-console"
        );
    }
}
