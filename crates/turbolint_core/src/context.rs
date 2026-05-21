use std::borrow::Cow;
use std::cell::RefCell;

use crate::diagnostic::{Diagnostic, Fix, Severity, Span};

pub struct RuleContext<'a> {
    source_text: &'a str,
    rule_id: &'static str,
    severity: Severity,
    diagnostics: &'a RefCell<Vec<Diagnostic>>,
}

impl<'a> RuleContext<'a> {
    pub fn new(
        source_text: &'a str,
        rule_id: &'static str,
        severity: Severity,
        diagnostics: &'a RefCell<Vec<Diagnostic>>,
    ) -> Self {
        Self {
            source_text,
            rule_id,
            severity,
            diagnostics,
        }
    }

    pub fn source_text(&self) -> &'a str {
        self.source_text
    }

    pub fn node_text(&self, node: &tree_sitter::Node) -> &'a str {
        &self.source_text[node.start_byte()..node.end_byte()]
    }

    pub fn report(&self, start: u32, end: u32, message: impl Into<Cow<'static, str>>) {
        self.report_inner(start, end, message.into(), None);
    }

    pub fn report_with_fix(
        &self,
        start: u32,
        end: u32,
        message: impl Into<Cow<'static, str>>,
        fix: Fix,
    ) {
        self.report_inner(start, end, message.into(), Some(fix));
    }

    fn report_inner(&self, start: u32, end: u32, message: Cow<'static, str>, fix: Option<Fix>) {
        self.diagnostics.borrow_mut().push(Diagnostic {
            rule_id: self.rule_id,
            message,
            severity: self.severity,
            span: Span { start, end },
            fix,
        });
    }
}
