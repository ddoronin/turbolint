use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoSparseArrays;

impl Rule for NoSparseArrays {
    fn name(&self) -> &'static str {
        "no-sparse-arrays"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() == "array" {
            // In tree-sitter-javascript, sparse array holes are represented
            // by having no node between commas. We detect this by looking for
            // consecutive "," tokens or "[" followed by ",".
            let mut cursor = node.walk();
            let children: Vec<_> = node.children(&mut cursor).collect();
            for i in 0..children.len() {
                if children[i].kind() == "," {
                    // Check if previous non-comma sibling is "[" or ","
                    if i == 0 {
                        continue;
                    }
                    let prev = &children[i - 1];
                    if prev.kind() == "," || prev.kind() == "[" {
                        ctx.report(
                            node.start_byte() as u32,
                            node.end_byte() as u32,
                            "Unexpected comma in middle of array.",
                        );
                        return;
                    }
                }
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
        assert!(lint(Box::new(NoSparseArrays), "var a = [1, 2, 3];").is_empty());
        assert!(lint(Box::new(NoSparseArrays), "var a = [];").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoSparseArrays), "var a = [1,,2];");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].rule_id, "no-sparse-arrays");

        let d = lint(Box::new(NoSparseArrays), "var a = [,,];");
        assert_eq!(d.len(), 1);
    }
}
