use tree_sitter::Node;

/// Walk the parent chain looking for an ancestor of the given kind.
pub fn find_ancestor<'a>(node: &Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut current = node.parent();
    while let Some(n) = current {
        if n.kind() == kind {
            return Some(n);
        }
        current = n.parent();
    }
    None
}

/// Check if this node is a function-like node.
pub fn is_function(node: &Node) -> bool {
    matches!(
        node.kind(),
        "function_declaration"
            | "function_expression"
            | "arrow_function"
            | "generator_function"
            | "generator_function_declaration"
            | "method_definition"
    )
}

/// Check if this node is a loop construct.
pub fn is_loop(node: &Node) -> bool {
    matches!(
        node.kind(),
        "for_statement" | "for_in_statement" | "while_statement" | "do_statement"
    )
}

/// Check if a node has any direct child of the given kind.
pub fn has_child_of_kind(node: &Node, kind: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == kind {
            return true;
        }
    }
    false
}

/// Find the first direct child of a given kind.
#[allow(clippy::manual_find)]
pub fn first_child_of_kind<'a>(node: &Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == kind {
            return Some(child);
        }
    }
    None
}

/// Count direct children of a given kind.
pub fn count_children_of_kind(node: &Node, kind: &str) -> usize {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|c| c.kind() == kind)
        .count()
}

/// Check if node is inside an ancestor of the given kind.
pub fn is_inside(node: &Node, kind: &str) -> bool {
    find_ancestor(node, kind).is_some()
}
