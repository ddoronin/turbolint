use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUnexpectedMultiline;

impl Rule for NoUnexpectedMultiline {
    fn name(&self) -> &'static str {
        "no-unexpected-multiline"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        match node.kind() {
            "call_expression" => check_call(node, ctx),
            "subscript_expression" => check_subscript(node, ctx),
            _ => {}
        }
        // Tagged template: check if a template_string follows an expression on a new line
        if node.kind() == "template_string" {
            if let Some(parent) = node.parent() {
                if parent.kind() == "call_expression" {
                    if let Some(func) = parent.child_by_field_name("function") {
                        if node.start_position().row > func.end_position().row {
                            ctx.report(
                                node.start_byte() as u32,
                                node.end_byte() as u32,
                                "Unexpected newline between template tag and template literal.",
                            );
                        }
                    }
                }
            }
        }
    }
}

fn check_call(node: &Node, ctx: &RuleContext) {
    if let Some(args) = node.child_by_field_name("arguments") {
        if let Some(func) = node.child_by_field_name("function") {
            // Skip if no arguments (0-arg calls are not ASI hazards)
            if args.named_child_count() == 0 {
                return;
            }
            // Skip optional chaining calls (?.)
            let source = ctx.source_text();
            let between_bytes = &source[func.end_byte()..args.start_byte()];
            if between_bytes.contains("?.") {
                return;
            }
            let open_paren_row = args.start_position().row;
            let before_paren = source[..args.start_byte()].trim_end();
            let last_char = before_paren.as_bytes().last().copied();
            let token_row = if matches!(last_char, Some(b'>') | Some(b')')) {
                let pos = before_paren.len() - 1;
                source[..=pos].matches('\n').count()
            } else {
                func.end_position().row
            };
            if open_paren_row > token_row {
                ctx.report(
                    args.start_byte() as u32,
                    args.end_byte() as u32,
                    "Unexpected newline between function and ( of function call.",
                );
            }
        }
    }
}

fn check_subscript(node: &Node, ctx: &RuleContext) {
    // subscript_expression: object[index]
    // Check if '[' starts on a new line after the object
    if let Some(obj) = node.child_by_field_name("object") {
        if let Some(index) = node.child_by_field_name("index") {
            // Skip optional chaining
            let source = ctx.source_text();
            let between = &source[obj.end_byte()..index.start_byte()];
            if between.contains("?.") {
                return;
            }
            // Find the '[' — it's between object end and index start
            let bracket_row = if let Some(bracket_offset) = between.find('[') {
                let abs_offset = obj.end_byte() + bracket_offset;
                source[..abs_offset].matches('\n').count()
            } else {
                index.start_position().row
            };
            if bracket_row > obj.end_position().row {
                ctx.report(
                    index.start_byte() as u32,
                    index.end_byte() as u32,
                    "Unexpected newline between object and [ of property access.",
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
        assert!(lint(Box::new(NoUnexpectedMultiline), "foo(1, 2);").is_empty());
    }
    #[test]
    fn valid_zero_args_multiline() {
        assert!(lint(Box::new(NoUnexpectedMultiline), "var a = b\nfoo()").is_empty());
    }
    #[test]
    fn valid_subscript_same_line() {
        assert!(lint(Box::new(NoUnexpectedMultiline), "var x = a[0];").is_empty());
    }
    #[test]
    fn invalid_subscript_multiline() {
        let d = lint(Box::new(NoUnexpectedMultiline), "var a = b\n[1, 2, 3].forEach(f)");
        assert_eq!(d.len(), 1);
    }
}
