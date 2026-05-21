use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoEmptyPattern;

impl Rule for NoEmptyPattern {
    fn name(&self) -> &'static str {
        "no-empty-pattern"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "object_pattern" | "array_pattern" => {
                if node.named_child_count() == 0 {
                    let kind = if node.kind() == "object_pattern" {
                        "object"
                    } else {
                        "array"
                    };
                    ctx.report(
                        node.start_byte() as u32,
                        node.end_byte() as u32,
                        format!("Unexpected empty {} pattern.", kind),
                    );
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(NoEmptyPattern), "var {a} = obj;").is_empty());
        assert!(lint(Box::new(NoEmptyPattern), "var [a] = arr;").is_empty());
    }
    #[test]
    fn invalid_object() {
        // Note: `var {} = obj;` may parse differently. Let's test with function param.
        let d = lint(Box::new(NoEmptyPattern), "function foo({}) {}");
        assert_eq!(d.len(), 1);
    }
    #[test]
    fn invalid_array() {
        let d = lint(Box::new(NoEmptyPattern), "function foo([]) {}");
        assert_eq!(d.len(), 1);
    }
}
