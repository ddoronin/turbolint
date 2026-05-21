use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoMisleadingCharacterClass;

impl Rule for NoMisleadingCharacterClass {
    fn name(&self) -> &'static str {
        "no-misleading-character-class"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "regex" {
            return;
        }
        let text = ctx.node_text(node);
        // Detect combining characters/surrogate pairs in character classes
        // Simple heuristic: check for multi-byte characters inside [...]
        let bytes = text.as_bytes();
        let mut in_class = false;
        let mut i = 1; // skip leading /
        while i < bytes.len() {
            if bytes[i] == b'/' && !in_class {
                break;
            }
            if bytes[i] == b'\\' {
                i += 2;
                continue;
            }
            if bytes[i] == b'[' {
                in_class = true;
            } else if bytes[i] == b']' {
                in_class = false;
            } else if in_class && bytes[i] > 0x7f {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Character class contains a composite character.",
                );
                return;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoMisleadingCharacterClass), "var re = /[abc]/;").is_empty());
    }
}
