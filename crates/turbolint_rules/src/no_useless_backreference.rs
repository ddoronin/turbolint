use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUselessBackreference;

impl Rule for NoUselessBackreference {
    fn name(&self) -> &'static str {
        "no-useless-backreference"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "regex" {
            return;
        }
        let text = ctx.node_text(node);
        let pattern = match text.strip_prefix('/') {
            Some(p) => match p.rfind('/') {
                Some(i) => &p[..i],
                None => return,
            },
            None => return,
        };

        // Track group positions and backreference positions
        let bytes = pattern.as_bytes();
        let mut groups_seen = 0usize;
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                let next = bytes[i + 1];
                if next.is_ascii_digit() && next != b'0' {
                    let digit = (next - b'0') as usize;
                    if digit > groups_seen {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            format!(
                                "Backreference '\\{}' will always match empty because group {} hasn't been defined yet.",
                                digit, digit
                            ),
                        );
                        return;
                    }
                }
                i += 2;
                continue;
            }
            if bytes[i] == b'(' {
                // Count as capture group if not (?:, (?=, (?!, (?<= etc.
                if i + 1 < bytes.len() && bytes[i + 1] != b'?' {
                    groups_seen += 1;
                } else if i + 2 < bytes.len() && bytes[i + 1] == b'?' && bytes[i + 2] == b'<' {
                    // Named group (?<name>) is also a capture group
                    if i + 3 < bytes.len() && bytes[i + 3] != b'=' && bytes[i + 3] != b'!' {
                        groups_seen += 1;
                    }
                }
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
        assert!(lint(Box::new(NoUselessBackreference), r"var re = /(a)\1/;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoUselessBackreference), r"var re = /\1(a)/;");
        assert_eq!(d.len(), 1);
    }
}
