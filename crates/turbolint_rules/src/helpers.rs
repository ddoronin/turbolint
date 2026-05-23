use tree_sitter::Node;

/// Unwrap parenthesized expressions to get the inner expression.
pub fn unwrap_parens<'a>(node: &'a Node<'a>) -> Node<'a> {
    let mut current = *node;
    while current.kind() == "parenthesized_expression" {
        if let Some(inner) = current.named_child(0) {
            current = inner;
        } else {
            break;
        }
    }
    current
}

/// Check if a node is a terminal statement (break/return/throw/continue).
pub fn is_terminal(node: &Node) -> bool {
    matches!(
        node.kind(),
        "return_statement" | "throw_statement" | "break_statement" | "continue_statement"
    )
}

/// Check if a text contains a fallthrough comment (case-insensitive).
pub fn has_fallthrough_comment(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("falls through")
        || lower.contains("fallthrough")
        || lower.contains("fall through")
}
