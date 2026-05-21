use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Warning => f.write_str("warning"),
            Severity::Error => f.write_str("error"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone)]
pub struct Fix {
    pub range: Span,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub rule_id: &'static str,
    pub message: Cow<'static, str>,
    pub severity: Severity,
    pub span: Span,
    pub fix: Option<Fix>,
}
