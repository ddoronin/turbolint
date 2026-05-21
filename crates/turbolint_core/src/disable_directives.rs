use crate::line_index::LineIndex;

/// The kind of disable directive.
#[derive(Debug, PartialEq)]
enum DirectiveKind {
    /// `eslint-disable-line` — suppresses the current line only.
    DisableLine,
    /// `eslint-disable-next-line` — suppresses the next line only.
    DisableNextLine,
    /// `/* eslint-disable */` — suppresses from this point until a matching enable or EOF.
    DisableBlock,
    /// `/* eslint-enable */` — re-enables rules disabled by a prior block disable.
    EnableBlock,
}

/// A parsed disable/enable directive from a comment.
#[derive(Debug)]
struct Directive {
    kind: DirectiveKind,
    /// 1-based line where the comment appears.
    line: usize,
    /// If empty, applies to all rules. Otherwise only listed rules.
    rules: Vec<String>,
}

/// Strips the `-- description` justification suffix from a directive value.
/// ESLint allows `-- some reason` after the rule list.
fn strip_description(s: &str) -> &str {
    // The description separator is two or more consecutive `-` characters
    // preceded by whitespace (or at the start of the string).
    // Handle string starting with "--" (no rules, just a description)
    if s.starts_with("--") {
        let after_dashes = s.strip_prefix("--").unwrap().trim_start_matches('-');
        if after_dashes.is_empty() || after_dashes.starts_with(|c: char| c.is_whitespace()) {
            return "";
        }
    }
    if let Some(pos) = s.find(" --") {
        let after_dashes = &s[pos + 3..];
        let first_non_dash = after_dashes.trim_start_matches('-');
        if first_non_dash.is_empty() || first_non_dash.starts_with(|c: char| c.is_whitespace()) {
            return s[..pos].trim_end();
        }
    }
    s
}

/// Parse a comma-separated list of rule names. Returns empty vec if no rules specified.
/// Handles quoted rule names (e.g., `'rule-name'` or `"rule-name"`).
fn parse_rule_list(s: &str) -> Vec<String> {
    let s = strip_description(s);
    if s.is_empty() {
        return Vec::new();
    }
    s.split(',')
        .map(|r| r.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|r| !r.is_empty())
        .collect()
}

/// Finds comment bodies in a single line of source text.
/// Returns an iterator of (comment_prefix, comment_body) where prefix is "//" or "/*".
fn find_comment_body<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    // Try // style
    if let Some(pos) = line.find(&format!("// {keyword}")) {
        let start = pos + 3;
        return Some(&line[start..]);
    }
    // Try /* style
    if let Some(pos) = line.find(&format!("/* {keyword}")) {
        let start = pos + 3;
        let body = &line[start..];
        // Strip trailing */
        let body = if let Some(end_pos) = body.find("*/") {
            &body[..end_pos]
        } else {
            body
        };
        return Some(body);
    }
    None
}

/// Parses all eslint disable/enable directives from source text.
fn parse_directives(source: &str, line_index: &LineIndex) -> Vec<Directive> {
    let mut directives = Vec::new();

    for (byte_offset, line_text) in LineIter::new(source) {
        let (current_line, _) = line_index.line_col(byte_offset as u32);

        // A single line may contain multiple directives (e.g., disable + enable).
        // We parse all of them, accumulating into `directives`.
        // We search through progressively shorter suffixes of the line.
        let mut search_from = 0usize;

        while search_from < line_text.len() {
            let remaining = &line_text[search_from..];

            // eslint-disable-next-line (check before eslint-disable to avoid prefix match)
            if let Some(body) = find_comment_body(remaining, "eslint-disable-next-line") {
                let rest = body.strip_prefix("eslint-disable-next-line").unwrap();
                let rest = rest.trim_end_matches("*/").trim();
                directives.push(Directive {
                    kind: DirectiveKind::DisableNextLine,
                    line: current_line,
                    rules: parse_rule_list(rest),
                });
                // Advance past this directive
                let pos = remaining.find("eslint-disable-next-line").unwrap();
                search_from += pos + "eslint-disable-next-line".len();
                continue;
            }

            // eslint-disable-line (check before eslint-disable to avoid prefix match)
            if let Some(body) = find_comment_body(remaining, "eslint-disable-line") {
                let rest = body.strip_prefix("eslint-disable-line").unwrap();
                let rest = rest.trim_end_matches("*/").trim();
                directives.push(Directive {
                    kind: DirectiveKind::DisableLine,
                    line: current_line,
                    rules: parse_rule_list(rest),
                });
                let pos = remaining.find("eslint-disable-line").unwrap();
                search_from += pos + "eslint-disable-line".len();
                continue;
            }

            // eslint-enable (check before eslint-disable to avoid prefix match)
            if let Some(body) = find_comment_body(remaining, "eslint-enable") {
                let rest = body.strip_prefix("eslint-enable").unwrap();
                let rest = rest.trim_end_matches("*/").trim();
                directives.push(Directive {
                    kind: DirectiveKind::EnableBlock,
                    line: current_line,
                    rules: parse_rule_list(rest),
                });
                let pos = remaining.find("eslint-enable").unwrap();
                search_from += pos + "eslint-enable".len();
                continue;
            }

            // eslint-disable (block)
            if let Some(body) = find_comment_body(remaining, "eslint-disable") {
                let rest = body.strip_prefix("eslint-disable").unwrap();
                if rest.is_empty()
                    || rest.starts_with(|c: char| c.is_whitespace())
                    || rest.starts_with("*/")
                {
                    let rest = rest.trim_end_matches("*/").trim();
                    directives.push(Directive {
                        kind: DirectiveKind::DisableBlock,
                        line: current_line,
                        rules: parse_rule_list(rest),
                    });
                }
                let pos = remaining.find("eslint-disable").unwrap();
                search_from += pos + "eslint-disable".len();
                continue;
            }

            break;
        }
    }

    directives
}

/// Iterator over lines of source text, yielding (byte_offset_of_line_start, line_text).
struct LineIter<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> LineIter<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.source.len() {
            return None;
        }
        let start = self.pos;
        match self.source[start..].find('\n') {
            Some(newline_offset) => {
                let end = start + newline_offset;
                self.pos = end + 1;
                Some((start, &self.source[start..end]))
            }
            None => {
                self.pos = self.source.len();
                Some((start, &self.source[start..]))
            }
        }
    }
}

/// Filters diagnostics, removing those suppressed by disable directives.
pub fn filter_disabled(
    diagnostics: Vec<crate::diagnostic::Diagnostic>,
    source: &str,
    line_index: &LineIndex,
) -> Vec<crate::diagnostic::Diagnostic> {
    let directives = parse_directives(source, line_index);
    if directives.is_empty() {
        return diagnostics;
    }

    diagnostics
        .into_iter()
        .filter(|d| {
            let (line, _) = line_index.line_col(d.span.start);
            !is_disabled(&directives, line, d.rule_id)
        })
        .collect()
}

/// Checks whether a given rule on a given line is suppressed by any directive.
fn is_disabled(directives: &[Directive], line: usize, rule_id: &str) -> bool {
    // Check line-scoped directives first (disable-line, disable-next-line)
    for d in directives {
        match d.kind {
            DirectiveKind::DisableLine => {
                if d.line == line && rule_matches(&d.rules, rule_id) {
                    return true;
                }
            }
            DirectiveKind::DisableNextLine => {
                if d.line + 1 == line && rule_matches(&d.rules, rule_id) {
                    return true;
                }
            }
            _ => {}
        }
    }

    // Check block-scoped directives (eslint-disable / eslint-enable)
    // Walk directives in order; track whether the rule is currently disabled.
    let mut disabled = false;
    for d in directives {
        // Only consider block directives that appear before or on this line
        if d.line > line {
            break;
        }
        match d.kind {
            DirectiveKind::DisableBlock => {
                if rule_matches(&d.rules, rule_id) {
                    disabled = true;
                }
            }
            DirectiveKind::EnableBlock => {
                if rule_matches(&d.rules, rule_id) {
                    disabled = false;
                }
            }
            _ => {}
        }
    }

    disabled
}

/// Returns true if the rule list is empty (matches all) or contains the given rule_id.
fn rule_matches(rules: &[String], rule_id: &str) -> bool {
    rules.is_empty() || rules.iter().any(|r| r == rule_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- strip_description tests ----

    #[test]
    fn strip_description_no_description() {
        assert_eq!(strip_description("no-debugger"), "no-debugger");
    }

    #[test]
    fn strip_description_with_double_dash() {
        assert_eq!(
            strip_description("no-debugger -- this is needed"),
            "no-debugger"
        );
    }

    #[test]
    fn strip_description_with_many_dashes() {
        assert_eq!(
            strip_description("no-debugger ----- long separator"),
            "no-debugger"
        );
    }

    #[test]
    fn strip_description_does_not_strip_single_dash() {
        // A single dash is part of a rule name like "no-debugger"
        assert_eq!(strip_description("no-debugger"), "no-debugger");
    }

    #[test]
    fn strip_description_empty() {
        assert_eq!(strip_description(""), "");
    }

    #[test]
    fn strip_description_only_dashes() {
        assert_eq!(strip_description(" --"), "");
    }

    // ---- parse_rule_list tests ----

    #[test]
    fn parse_rule_list_empty() {
        assert!(parse_rule_list("").is_empty());
    }

    #[test]
    fn parse_rule_list_single() {
        assert_eq!(parse_rule_list("no-debugger"), vec!["no-debugger"]);
    }

    #[test]
    fn parse_rule_list_multiple() {
        assert_eq!(
            parse_rule_list("no-console, no-debugger"),
            vec!["no-console", "no-debugger"]
        );
    }

    #[test]
    fn parse_rule_list_with_description() {
        assert_eq!(
            parse_rule_list("no-console, no-debugger -- needed for production"),
            vec!["no-console", "no-debugger"]
        );
    }

    #[test]
    fn parse_rule_list_quoted_rules() {
        assert_eq!(
            parse_rule_list("'no-console', \"no-debugger\""),
            vec!["no-console", "no-debugger"]
        );
    }

    #[test]
    fn parse_rule_list_extra_whitespace() {
        assert_eq!(
            parse_rule_list("  no-console ,  no-debugger  "),
            vec!["no-console", "no-debugger"]
        );
    }

    // ---- parse_directives: disable-line ----

    #[test]
    fn parse_disable_line_all_rules() {
        let src = "debugger; // eslint-disable-line\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].kind, DirectiveKind::DisableLine);
        assert_eq!(directives[0].line, 1);
        assert!(directives[0].rules.is_empty());
    }

    #[test]
    fn parse_disable_line_specific_rule() {
        let src = "debugger; // eslint-disable-line no-debugger\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].rules, vec!["no-debugger"]);
    }

    #[test]
    fn parse_disable_line_block_comment() {
        let src = "debugger; /* eslint-disable-line */\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].kind, DirectiveKind::DisableLine);
        assert!(directives[0].rules.is_empty());
    }

    // ---- parse_directives: disable-next-line ----

    #[test]
    fn parse_disable_next_line_all_rules() {
        let src = "// eslint-disable-next-line\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].kind, DirectiveKind::DisableNextLine);
        assert_eq!(directives[0].line, 1);
        assert!(directives[0].rules.is_empty());
    }

    #[test]
    fn parse_disable_next_line_specific_rule() {
        let src = "// eslint-disable-next-line no-debugger\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives[0].rules, vec!["no-debugger"]);
    }

    #[test]
    fn parse_disable_next_line_multiple_rules() {
        let src = "// eslint-disable-next-line no-console, no-debugger\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives[0].rules, vec!["no-console", "no-debugger"]);
    }

    #[test]
    fn parse_disable_next_line_block_comment() {
        let src = "/* eslint-disable-next-line no-debugger */\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].kind, DirectiveKind::DisableNextLine);
        assert_eq!(directives[0].rules, vec!["no-debugger"]);
    }

    // ---- parse_directives: block disable/enable ----

    #[test]
    fn parse_block_disable_all() {
        let src = "/* eslint-disable */\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].kind, DirectiveKind::DisableBlock);
        assert!(directives[0].rules.is_empty());
    }

    #[test]
    fn parse_block_disable_specific() {
        let src = "/* eslint-disable no-debugger */\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].kind, DirectiveKind::DisableBlock);
        assert_eq!(directives[0].rules, vec!["no-debugger"]);
    }

    #[test]
    fn parse_block_enable_all() {
        let src = "/* eslint-disable */\ndebugger;\n/* eslint-enable */\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 2);
        assert_eq!(directives[0].kind, DirectiveKind::DisableBlock);
        assert_eq!(directives[1].kind, DirectiveKind::EnableBlock);
        assert_eq!(directives[1].line, 3);
    }

    #[test]
    fn parse_block_enable_specific() {
        let src = "/* eslint-disable no-debugger, no-console */\n/* eslint-enable no-debugger */\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 2);
        assert_eq!(directives[0].rules, vec!["no-debugger", "no-console"]);
        assert_eq!(directives[1].kind, DirectiveKind::EnableBlock);
        assert_eq!(directives[1].rules, vec!["no-debugger"]);
    }

    // ---- parse_directives: description/justification ----

    #[test]
    fn parse_with_description_line() {
        let src = "debugger; // eslint-disable-line no-debugger -- needed for dev\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].rules, vec!["no-debugger"]);
    }

    #[test]
    fn parse_with_description_next_line() {
        let src = "// eslint-disable-next-line no-debugger -- temporarily needed\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives[0].rules, vec!["no-debugger"]);
    }

    #[test]
    fn parse_with_description_block_disable() {
        let src = "/* eslint-disable no-debugger -- legacy code */\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives[0].rules, vec!["no-debugger"]);
    }

    #[test]
    fn parse_with_description_no_rules() {
        let src = "// eslint-disable-next-line -- suppress everything here\ndebugger;\n";
        let idx = LineIndex::new(src);
        let directives = parse_directives(src, &idx);
        assert_eq!(directives.len(), 1);
        assert!(directives[0].rules.is_empty());
    }
}
