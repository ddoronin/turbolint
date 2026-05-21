use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;
pub struct NoThisBeforeSuper;
impl Rule for NoThisBeforeSuper {
    fn name(&self) -> &'static str {
        "no-this-before-super"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "this" {
            return;
        }
        // Check if inside a constructor of a derived class, before super()
        let mut current = node.parent();
        while let Some(p) = current {
            if p.kind() == "method_definition" {
                if let Some(name) = p.child_by_field_name("name") {
                    if ctx.node_text(&name) == "constructor" {
                        // Check class has extends
                        if let Some(class) = find_class(&p) {
                            let has_extends = {
                                let mut c = class.walk();
                                let found = class
                                    .children(&mut c)
                                    .any(|ch| ch.kind() == "class_heritage");
                                found
                            };
                            if has_extends {
                                // Check if super() comes before this in source order
                                if let Some(body) = p.child_by_field_name("body") {
                                    if !super_before_offset(&body, node.start_byte()) {
                                        ctx.report(
                                            node.start_byte() as u32,
                                            node.end_byte() as u32,
                                            "'this' before 'super()'.",
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                return;
            }
            if matches!(
                p.kind(),
                "function_declaration" | "function_expression" | "arrow_function"
            ) {
                return;
            }
            current = p.parent();
        }
    }
}
fn find_class<'a>(node: &Node<'a>) -> Option<Node<'a>> {
    let mut current = node.parent();
    while let Some(p) = current {
        if matches!(p.kind(), "class_declaration" | "class") {
            return Some(p);
        }
        current = p.parent();
    }
    None
}
fn super_before_offset(node: &Node, offset: usize) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.start_byte() >= offset {
            return false;
        }
        if child.kind() == "call_expression" {
            if let Some(f) = child.child_by_field_name("function") {
                if f.kind() == "super" {
                    return true;
                }
            }
        }
        if super_before_offset(&child, offset) {
            return true;
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
        assert!(lint(
            Box::new(NoThisBeforeSuper),
            "class Foo { constructor() { this.x = 1; } }"
        )
        .is_empty());
    }
}
