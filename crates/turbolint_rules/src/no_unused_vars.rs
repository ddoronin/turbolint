use std::collections::HashSet;
use turbolint_core::context::RuleContext;
use turbolint_core::diagnostic::Severity;
use turbolint_core::Rule;
use tree_sitter::Node;

pub struct NoUnusedVars;

impl Rule for NoUnusedVars {
    fn name(&self) -> &'static str {
        "no-unused-vars"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "program" {
            return;
        }
        let source = ctx.source_text();
        let mut decls: Vec<Decl> = Vec::new();
        let mut refs: HashSet<&str> = HashSet::new();

        collect(node, source, &mut decls, &mut refs);

        for d in &decls {
            if d.exported {
                continue;
            }
            if !refs.contains(d.name.as_str()) {
                ctx.report(
                    d.start,
                    d.end,
                    format!("'{}' is defined but never used.", d.name),
                );
            }
        }
    }
}

struct Decl {
    name: String,
    start: u32,
    end: u32,
    exported: bool,
}

fn collect<'a>(
    node: &Node<'a>,
    source: &'a str,
    decls: &mut Vec<Decl>,
    refs: &mut HashSet<&'a str>,
) {
    match node.kind() {
        "identifier" => {
            let text = &source[node.start_byte()..node.end_byte()];
            match classify(node) {
                IdRole::Decl => {
                    decls.push(Decl {
                        name: text.to_string(),
                        start: node.start_byte() as u32,
                        end: node.end_byte() as u32,
                        exported: is_exported(node),
                    });
                }
                IdRole::Ref => {
                    refs.insert(text);
                }
                IdRole::Skip => {}
            }
        }
        "shorthand_property_identifier_pattern" => {
            let text = &source[node.start_byte()..node.end_byte()];
            decls.push(Decl {
                name: text.to_string(),
                start: node.start_byte() as u32,
                end: node.end_byte() as u32,
                exported: is_exported(node),
            });
        }
        "shorthand_property_identifier" => {
            refs.insert(&source[node.start_byte()..node.end_byte()]);
        }
        "type_identifier" => {
            let text = &source[node.start_byte()..node.end_byte()];
            // Type declarations (interface Foo, type Foo = ...) are declarations.
            // All other type_identifier usages are references.
            if is_type_decl(node) {
                decls.push(Decl {
                    name: text.to_string(),
                    start: node.start_byte() as u32,
                    end: node.end_byte() as u32,
                    exported: is_exported(node),
                });
            } else {
                refs.insert(text);
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect(&child, source, decls, refs);
    }
}

enum IdRole {
    Decl,
    Ref,
    Skip,
}

fn classify(node: &Node) -> IdRole {
    let parent = match node.parent() {
        Some(p) => p,
        None => return IdRole::Ref,
    };

    match parent.kind() {
        // const x = 1;  /  let x;  /  var x;
        "variable_declarator" => {
            if is_field(&parent, "name", node) {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // function foo() {}  /  class Foo {}  /  enum E {}
        "function_declaration" | "generator_function_declaration" | "class_declaration"
        | "enum_declaration" | "abstract_class_declaration" => {
            if is_field(&parent, "name", node) {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // Function/class expressions: name is self-reference only, not outer-scope declaration
        "function_expression" | "generator_function" | "class" => {
            if is_field(&parent, "name", node) {
                IdRole::Skip
            } else {
                IdRole::Ref
            }
        }

        // Arrow function single parameter: x => x + 1
        "arrow_function" => {
            if is_field(&parent, "parameter", node) {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // import Foo from 'mod'  (default import, identifier is direct child of import_clause)
        "import_clause" => IdRole::Decl,

        // import * as Foo from 'mod'
        "namespace_import" => IdRole::Decl,

        // import { Foo } from 'mod'  /  import { Foo as Bar } from 'mod'
        "import_specifier" => {
            if let Some(alias) = parent.child_by_field_name("alias") {
                if alias.id() == node.id() {
                    IdRole::Decl // alias is the local binding
                } else {
                    IdRole::Skip // name is the remote export name
                }
            } else if is_field(&parent, "name", node) {
                IdRole::Decl
            } else {
                IdRole::Skip
            }
        }

        // export { x }  /  export { x as y }
        "export_specifier" => {
            if parent
                .child_by_field_name("alias")
                .is_some_and(|a| a.id() == node.id())
            {
                IdRole::Skip // exported name, not a local reference
            } else {
                IdRole::Ref // local name being exported — counts as "used"
            }
        }

        // TypeScript function parameters: (x: number)
        "required_parameter" | "optional_parameter" => {
            if is_field(&parent, "name", node) || is_field(&parent, "pattern", node) {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // JS-style parameters without type annotations
        "formal_parameters" => IdRole::Decl,

        // catch (e) { ... }
        "catch_clause" => {
            if is_field(&parent, "parameter", node) {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // [a, b] = arr  /  ...rest
        "array_pattern" | "rest_pattern" => IdRole::Decl,

        // { key: value } in destructuring — value is the binding
        "pair_pattern" => {
            if is_field(&parent, "value", node) {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // x = defaultVal in destructuring
        "assignment_pattern" => {
            if is_field(&parent, "left", node) {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // for (const x of items) — declaration only with let/const/var
        "for_in_statement" => {
            if is_field(&parent, "left", node) && parent.child_by_field_name("kind").is_some() {
                IdRole::Decl
            } else {
                IdRole::Ref
            }
        }

        // Everything else: reference
        _ => IdRole::Ref,
    }
}

fn is_type_decl(node: &Node) -> bool {
    node.parent().is_some_and(|p| {
        matches!(
            p.kind(),
            "interface_declaration" | "type_alias_declaration"
        ) && is_field(&p, "name", node)
    })
}

fn is_field(parent: &Node, field: &str, node: &Node) -> bool {
    parent
        .child_by_field_name(field)
        .is_some_and(|f| f.id() == node.id())
}

fn is_exported(node: &Node) -> bool {
    let mut current = node.parent();
    while let Some(n) = current {
        match n.kind() {
            "export_statement" => return true,
            "program" | "statement_block" => return false,
            _ => current = n.parent(),
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid_used_var() {
        assert!(lint(Box::new(NoUnusedVars), "var x = 1; console.log(x);").is_empty());
    }

    #[test]
    fn valid_exported() {
        assert!(lint(Box::new(NoUnusedVars), "export var x = 1;").is_empty());
    }

    #[test]
    fn valid_export_clause() {
        assert!(lint(Box::new(NoUnusedVars), "var x = 1; export { x };").is_empty());
    }

    #[test]
    fn valid_used_function() {
        assert!(lint(
            Box::new(NoUnusedVars),
            "function foo() { return 1; } foo();"
        )
        .is_empty());
    }

    #[test]
    fn valid_used_destructured() {
        assert!(lint(
            Box::new(NoUnusedVars),
            "var { a, b } = obj; console.log(a, b);"
        )
        .is_empty());
    }

    #[test]
    fn valid_used_in_binary() {
        assert!(lint(Box::new(NoUnusedVars), "var x = 1; var y = x + 1; console.log(y);").is_empty());
    }

    #[test]
    fn invalid_unused_var() {
        let d = lint(Box::new(NoUnusedVars), "var x = 1;");
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'x'"));
    }

    #[test]
    fn invalid_unused_function() {
        let d = lint(Box::new(NoUnusedVars), "function foo() { return 1; }");
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'foo'"));
    }

    #[test]
    fn invalid_unused_import() {
        let d = lint(Box::new(NoUnusedVars), "import { Foo } from 'bar';");
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'Foo'"));
    }

    #[test]
    fn invalid_unused_default_import() {
        let d = lint(Box::new(NoUnusedVars), "import Foo from 'bar';");
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'Foo'"));
    }

    #[test]
    fn invalid_unused_namespace_import() {
        let d = lint(Box::new(NoUnusedVars), "import * as Foo from 'bar';");
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'Foo'"));
    }

    #[test]
    fn invalid_partial_destructuring() {
        let d = lint(Box::new(NoUnusedVars), "var { a, b } = obj; console.log(a);");
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'b'"));
    }

    #[test]
    fn invalid_unused_param() {
        let d = lint(Box::new(NoUnusedVars), "function foo(x) { return 1; } foo();");
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'x'"));
    }

    #[test]
    fn valid_renamed_import() {
        assert!(lint(
            Box::new(NoUnusedVars),
            "import { Foo as Bar } from 'mod'; console.log(Bar);"
        )
        .is_empty());
    }

    #[test]
    fn invalid_renamed_import_unused() {
        let d = lint(
            Box::new(NoUnusedVars),
            "import { Foo as Bar } from 'mod';",
        );
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'Bar'"));
    }

    #[test]
    fn valid_export_renamed() {
        assert!(lint(
            Box::new(NoUnusedVars),
            "var x = 1; export { x as y };"
        )
        .is_empty());
    }

    #[test]
    fn valid_catch_used() {
        assert!(lint(
            Box::new(NoUnusedVars),
            "try {} catch (e) { console.log(e); }"
        )
        .is_empty());
    }

    #[test]
    fn valid_array_destructuring() {
        assert!(lint(
            Box::new(NoUnusedVars),
            "var [a, b] = arr; console.log(a, b);"
        )
        .is_empty());
    }

    #[test]
    fn invalid_array_destructuring_partial() {
        let d = lint(
            Box::new(NoUnusedVars),
            "var [a, b] = arr; console.log(a);",
        );
        assert_eq!(d.len(), 1);
        assert!(d[0].message.contains("'b'"));
    }
}
