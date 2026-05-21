use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoIrregularWhitespace;

const IRREGULAR_WS: &[char] = &[
    '\u{000B}', '\u{000C}', '\u{00A0}', '\u{FEFF}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
    '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
    '\u{202F}', '\u{205F}', '\u{3000}',
];

impl Rule for NoIrregularWhitespace {
    fn name(&self) -> &'static str {
        "no-irregular-whitespace"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "program" {
            return;
        }
        let source = ctx.source_text();
        for (i, ch) in source.char_indices() {
            if IRREGULAR_WS.contains(&ch) {
                ctx.report(
                    i as u32,
                    (i + ch.len_utf8()) as u32,
                    "Irregular whitespace not allowed.",
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoIrregularWhitespace), "var x = 1;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoIrregularWhitespace), "var x\u{00A0}= 1;");
        assert_eq!(d.len(), 1);
    }
}
