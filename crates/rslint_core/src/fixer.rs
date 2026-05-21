use crate::diagnostic::Diagnostic;

pub struct ApplyResult {
    pub output: String,
    pub fixed: bool,
    pub remaining: Vec<Diagnostic>,
}

pub fn apply_fixes(source: &str, diagnostics: Vec<Diagnostic>) -> ApplyResult {
    let (mut fixable, unfixable): (Vec<Diagnostic>, Vec<Diagnostic>) =
        diagnostics.into_iter().partition(|d| d.fix.is_some());

    if fixable.is_empty() {
        return ApplyResult {
            output: source.to_string(),
            fixed: false,
            remaining: unfixable,
        };
    }

    // Sort by fix range start ascending, then end ascending
    fixable.sort_by(|a, b| {
        let fa = a.fix.as_ref().unwrap();
        let fb = b.fix.as_ref().unwrap();
        fa.range.start.cmp(&fb.range.start).then(fa.range.end.cmp(&fb.range.end))
    });

    let mut output = String::with_capacity(source.len());
    let mut last_end: u32 = 0;
    let mut remaining = unfixable;

    for diag in fixable {
        let fix = diag.fix.as_ref().unwrap();
        if fix.range.start < last_end {
            // Overlapping fix — skip it, keep as remaining diagnostic
            remaining.push(diag);
            continue;
        }
        output.push_str(&source[last_end as usize..fix.range.start as usize]);
        output.push_str(&fix.text);
        last_end = fix.range.end;
    }

    output.push_str(&source[last_end as usize..]);

    ApplyResult {
        output,
        fixed: true,
        remaining,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::{Fix, Severity, Span};
    use std::borrow::Cow;

    fn make_diag(start: u32, end: u32, text: &str) -> Diagnostic {
        Diagnostic {
            rule_id: "test",
            message: Cow::Borrowed("test"),
            severity: Severity::Error,
            span: Span { start, end },
            fix: Some(Fix {
                range: Span { start, end },
                text: text.to_string(),
            }),
        }
    }

    fn make_unfixable(start: u32, end: u32) -> Diagnostic {
        Diagnostic {
            rule_id: "test",
            message: Cow::Borrowed("test"),
            severity: Severity::Error,
            span: Span { start, end },
            fix: None,
        }
    }

    #[test]
    fn no_fixes() {
        let result = apply_fixes("hello", vec![make_unfixable(0, 5)]);
        assert!(!result.fixed);
        assert_eq!(result.output, "hello");
        assert_eq!(result.remaining.len(), 1);
    }

    #[test]
    fn single_fix() {
        // Replace "var" with "let"
        let result = apply_fixes("var x = 1;", vec![make_diag(0, 3, "let")]);
        assert!(result.fixed);
        assert_eq!(result.output, "let x = 1;");
    }

    #[test]
    fn two_non_overlapping() {
        let result = apply_fixes(
            "var x = 1;; var y = 2;;",
            vec![make_diag(10, 11, ""), make_diag(22, 23, "")],
        );
        assert!(result.fixed);
        assert_eq!(result.output, "var x = 1; var y = 2;");
    }

    #[test]
    fn overlapping_second_skipped() {
        let result = apply_fixes(
            "abcdefgh",
            vec![make_diag(1, 4, "XX"), make_diag(3, 6, "YY")],
        );
        assert!(result.fixed);
        assert_eq!(result.output, "aXXefgh");
        assert_eq!(result.remaining.len(), 1);
    }

    #[test]
    fn deletion() {
        let result = apply_fixes("a;b", vec![make_diag(1, 2, "")]);
        assert!(result.fixed);
        assert_eq!(result.output, "ab");
    }

    #[test]
    fn insertion_zero_width() {
        let result = apply_fixes(".5", vec![make_diag(0, 0, "0")]);
        assert!(result.fixed);
        assert_eq!(result.output, "0.5");
    }

    #[test]
    fn fix_at_end() {
        let result = apply_fixes("1.", vec![make_diag(2, 2, "0")]);
        assert!(result.fixed);
        assert_eq!(result.output, "1.0");
    }
}
