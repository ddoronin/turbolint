use tree_sitter::{Parser, Tree};

pub use tree_sitter;

pub fn parse(source: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .expect("failed to load JavaScript grammar");
    parser.parse(source, None).expect("parsing failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_debugger_statement() {
        let tree = parse("debugger;");
        let root = tree.root_node();
        assert!(!root.has_error());
        assert_eq!(root.child(0).unwrap().kind(), "debugger_statement");
    }

    #[test]
    fn parse_var_declaration() {
        let tree = parse("var x = 1;");
        let root = tree.root_node();
        assert!(!root.has_error());
        assert_eq!(root.child(0).unwrap().kind(), "variable_declaration");
    }
}
