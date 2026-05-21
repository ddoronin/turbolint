use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoLossOfPrecision;

impl Rule for NoLossOfPrecision {
    fn name(&self) -> &'static str {
        "no-loss-of-precision"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "number" {
            return;
        }
        let text = ctx.node_text(node);
        // Skip hex, octal, binary literals
        if text.starts_with("0x")
            || text.starts_with("0X")
            || text.starts_with("0o")
            || text.starts_with("0O")
            || text.starts_with("0b")
            || text.starts_with("0B")
        {
            return;
        }
        let clean = text.replace('_', "");
        if let Ok(val) = clean.parse::<f64>() {
            // Round-trip: parse to f64 then format back and re-parse
            let formatted = format!("{}", val);
            if let Ok(reparsed) = formatted.parse::<f64>() {
                if val != reparsed || (val == reparsed && loses_precision(&clean)) {
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        "This number literal will lose precision at runtime.",
                    );
                }
            }
        }
    }
}

fn loses_precision(text: &str) -> bool {
    // Integer that exceeds Number.MAX_SAFE_INTEGER
    if !text.contains('.') && !text.contains('e') && !text.contains('E') {
        if let Ok(val) = text.parse::<i128>() {
            return !(-9007199254740991..=9007199254740991).contains(&val);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoLossOfPrecision), "var x = 12345;").is_empty());
        assert!(lint(Box::new(NoLossOfPrecision), "var x = 1.5;").is_empty());
        assert!(lint(Box::new(NoLossOfPrecision), "var x = 9007199254740991;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoLossOfPrecision), "var x = 9007199254740993;");
        assert_eq!(d.len(), 1);
    }
}
