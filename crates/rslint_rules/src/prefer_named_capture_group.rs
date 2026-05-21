use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct PreferNamedCaptureGroup;

impl Rule for PreferNamedCaptureGroup {
    fn name(&self) -> &'static str {
        "prefer-named-capture-group"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
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
        // Look for unnamed capture groups: ( not followed by ?
        let bytes = pattern.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'\\' {
                i += 2;
                continue;
            }
            if bytes[i] == b'(' && i + 1 < bytes.len() && bytes[i + 1] != b'?' {
                ctx.report(
                    node.start_byte() as u32,
                    node.end_byte() as u32,
                    "Capture group is not named.",
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
        assert!(lint(
            Box::new(PreferNamedCaptureGroup),
            "var re = /(?<name>foo)/;"
        )
        .is_empty());
        assert!(lint(Box::new(PreferNamedCaptureGroup), "var re = /foo/;").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(PreferNamedCaptureGroup), "var re = /(foo)/;");
        assert_eq!(d.len(), 1);
    }
}
