use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct NoDupeArgs;

impl Rule for NoDupeArgs {
    fn name(&self) -> &'static str {
        "no-dupe-args"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "formal_parameters" {
            return;
        }
        let mut names: Vec<String> = Vec::new();
        let mut cursor = node.walk();
        for param in node.named_children(&mut cursor) {
            let name = if param.kind() == "identifier" {
                Some(ctx.node_text(&param).to_string())
            } else if param.kind() == "assignment_pattern" {
                param
                    .child_by_field_name("left")
                    .filter(|l| l.kind() == "identifier")
                    .map(|l| ctx.node_text(&l).to_string())
            } else {
                None
            };
            if let Some(n) = name {
                if names.contains(&n) {
                    ctx.report(
                        param.start_byte() as u32,
                        param.end_byte() as u32,
                        format!("Duplicate parameter name '{}'.", n),
                    );
                } else {
                    names.push(n);
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
        assert!(lint(Box::new(NoDupeArgs), "function foo(a, b, c) {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(NoDupeArgs), "function foo(a, b, a) {}");
        assert_eq!(d.len(), 1);
    }
}
