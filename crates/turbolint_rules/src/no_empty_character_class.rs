use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoEmptyCharacterClass;

impl Rule for NoEmptyCharacterClass {
    fn name(&self) -> &'static str {
        "no-empty-character-class"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "regex" {
            return;
        }
        let text = ctx.node_text(node);
        // Simple check: look for [] in the regex pattern (not [^...] etc.)
        let bytes = text.as_bytes();
        let mut i = 1; // skip leading /
        while i < bytes.len() {
            if bytes[i] == b'/' && !is_escaped(bytes, i) {
                break;
            }
            if bytes[i] == b'['
                && !is_escaped(bytes, i)
                && i + 1 < bytes.len()
                && bytes[i + 1] == b']'
            {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Empty class.",
                );
                return;
            }
            i += 1;
        }
    }
}

fn is_escaped(bytes: &[u8], pos: usize) -> bool {
    if pos == 0 {
        return false;
    }
    let mut backslash_count = 0;
    let mut i = pos - 1;
    loop {
        if bytes[i] == b'\\' {
            backslash_count += 1;
        } else {
            break;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    backslash_count % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoEmptyCharacterClass), "var re = /[abc]/;").is_empty());
        assert!(lint(Box::new(NoEmptyCharacterClass), r"var re = /\[\]/;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoEmptyCharacterClass), "var re = /[]/;");
        assert_eq!(d.len(), 1);
    }
}
