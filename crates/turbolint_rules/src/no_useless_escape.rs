use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessEscape;

// Valid escapes in strings (ESLint spec): \\ \n \r \t \b \f \v \0 \x \u \' \"
// Plus line continuation characters (CR, LF, LS, PS)
const VALID_STRING_ESCAPES: &[u8] = b"\\nrtbfv0xu'\"";

impl Rule for NoUselessEscape {
    fn name(&self) -> &'static str {
        "no-useless-escape"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "string" => check_string(node, ctx),
            "template_string" => check_template(node, ctx),
            _ => {}
        }
    }
}

fn check_string(node: &Node, ctx: &RuleContext) {
    let text = ctx.node_text(node);
    let bytes = text.as_bytes();
    if bytes.len() < 2 {
        return;
    }
    let quote = bytes[0];
    let mut i = 1;
    while i < bytes.len() - 1 {
        if bytes[i] == b'\\' && i + 1 < bytes.len() - 1 {
            let next = bytes[i + 1];
            if next == quote
                || VALID_STRING_ESCAPES.contains(&next)
                || next.is_ascii_digit()
                || next == b'\n'
                || next == b'\r'
            {
                // valid escape
            } else {
                // Check for unicode line separators (LS: U+2028, PS: U+2029)
                // In UTF-8: E2 80 A8 and E2 80 A9
                if next == 0xE2
                    && i + 3 < bytes.len() - 1
                    && bytes[i + 2] == 0x80
                    && (bytes[i + 3] == 0xA8 || bytes[i + 3] == 0xA9)
                {
                    i += 4;
                    continue;
                }
                ctx.report(
                    (node.start_byte() + i) as u32,
                    (node.start_byte() + i + 2) as u32,
                    format!("Unnecessary escape character: \\{}.", next as char),
                );
            }
            i += 2;
            continue;
        }
        i += 1;
    }
}

fn check_template(node: &Node, ctx: &RuleContext) {
    let text = ctx.node_text(node);
    let bytes = text.as_bytes();
    if bytes.len() < 2 {
        return;
    }
    // Template strings: valid escapes are same as strings plus ` and $
    // Skip template substitutions (${...})
    let mut i = 1; // skip opening backtick
    let mut in_substitution = 0u32;
    while i < bytes.len() - 1 {
        if bytes[i] == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
            in_substitution += 1;
            i += 2;
            continue;
        }
        if in_substitution > 0 {
            if bytes[i] == b'{' {
                in_substitution += 1;
            } else if bytes[i] == b'}' {
                in_substitution -= 1;
            }
            i += 1;
            continue;
        }
        if bytes[i] == b'\\' && i + 1 < bytes.len() - 1 {
            let next = bytes[i + 1];
            if next == b'`'
                || next == b'$'
                || VALID_STRING_ESCAPES.contains(&next)
                || next.is_ascii_digit()
                || next == b'\n'
                || next == b'\r'
            {
                // valid escape
            } else {
                ctx.report(
                    (node.start_byte() + i) as u32,
                    (node.start_byte() + i + 2) as u32,
                    format!("Unnecessary escape character: \\{}.", next as char),
                );
            }
            i += 2;
            continue;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoUselessEscape), r#"var x = "hello\n";"#).is_empty());
        assert!(lint(Box::new(NoUselessEscape), r#"var x = "hello";"#).is_empty());
    }
    #[test]
    fn valid_template() {
        assert!(lint(Box::new(NoUselessEscape), r#"var x = `hello\n`;"#).is_empty());
        assert!(lint(Box::new(NoUselessEscape), r"var x = `hello\$`;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUselessEscape), r#"var x = "he\llo";"#);
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_uppercase_u() {
        // \U is NOT a valid escape in ESLint
        let d = lint(Box::new(NoUselessEscape), r#"var x = "\Ufoo";"#);
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_template() {
        let d = lint(Box::new(NoUselessEscape), r#"var x = `he\llo`;"#);
        assert_eq!(d.len(), 1);
    }
}
