use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;
pub struct ConstructorSuper;
impl Rule for ConstructorSuper {
    fn name(&self) -> &'static str {
        "constructor-super"
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "class_declaration" {
            return;
        }
        let has_extends = node.child_by_field_name("heritage").is_some() || {
            let mut c = node.walk();
            let found = node
                .children(&mut c)
                .any(|ch| ch.kind() == "class_heritage");
            found
        };
        if !has_extends {
            return;
        }
        let body = match node.child_by_field_name("body") {
            Some(b) => b,
            None => return,
        };
        let mut cursor = body.walk();
        for child in body.named_children(&mut cursor) {
            if child.kind() == "method_definition" {
                if let Some(name) = child.child_by_field_name("name") {
                    if ctx.node_text(&name) == "constructor" {
                        if let Some(fn_body) = child.child_by_field_name("body") {
                            if !has_super_call(&fn_body) {
                                ctx.report(
                                    child.start_byte() as u32,
                                    child.end_byte() as u32,
                                    "Expected super() call in constructor of derived class.",
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
fn has_super_call(node: &Node) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "call_expression" {
            if let Some(f) = child.child_by_field_name("function") {
                if f.kind() == "super" {
                    return true;
                }
            }
        }
        if matches!(
            child.kind(),
            "function_declaration" | "function_expression" | "arrow_function"
        ) {
            continue;
        }
        if has_super_call(&child) {
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
        assert!(lint(Box::new(ConstructorSuper), "class Foo {}").is_empty());
    }
}
