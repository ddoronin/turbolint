use rslint_core::context::RuleContext;
use rslint_core::diagnostic::Severity;
use rslint_core::Rule;
use tree_sitter::Node;

pub struct MaxClassesPerFile;

const DEFAULT_MAX: usize = 1;

impl Rule for MaxClassesPerFile {
    fn name(&self) -> &'static str {
        "max-classes-per-file"
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn on_node(&self, node: &Node, ctx: &RuleContext) {
        if node.kind() != "program" {
            return;
        }
        let count = count_classes(node);
        if count > DEFAULT_MAX {
            ctx.report(
                node.start_byte() as u32,
                node.end_byte() as u32,
                format!(
                    "File has too many classes ({}). Maximum allowed is {}.",
                    count, DEFAULT_MAX
                ),
            );
        }
    }
}

fn count_classes(node: &Node) -> usize {
    let mut count = 0;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "class_declaration" {
            count += 1;
        }
        count += count_classes(&child);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::lint;

    #[test]
    fn valid() {
        assert!(lint(Box::new(MaxClassesPerFile), "class Foo {}").is_empty());
    }
    #[test]
    fn invalid() {
        let d = lint(Box::new(MaxClassesPerFile), "class Foo {} class Bar {}");
        assert_eq!(d.len(), 1);
    }
}
