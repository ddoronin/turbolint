use std::borrow::Cow;
use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

impl Serialize for Severity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Severity::Warning => serializer.serialize_u8(1),
            Severity::Error => serializer.serialize_u8(2),
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Warning => f.write_str("warning"),
            Severity::Error => f.write_str("error"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Fix {
    pub range: Span,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub rule_id: &'static str,
    pub message: Cow<'static, str>,
    pub severity: Severity,
    pub span: Span,
    pub fix: Option<Fix>,
}
