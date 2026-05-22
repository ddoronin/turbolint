use tree_sitter::{Parser, Tree};

pub use tree_sitter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    JavaScript,
    TypeScript,
    Tsx,
}

impl Language {
    /// Detect language from a file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "js" | "mjs" | "cjs" => Some(Self::JavaScript),
            "ts" | "mts" | "cts" => Some(Self::TypeScript),
            "tsx" => Some(Self::Tsx),
            _ => None,
        }
    }
}

pub fn parse(source: &str) -> Tree {
    parse_lang(source, Language::JavaScript)
}

pub fn parse_lang(source: &str, lang: Language) -> Tree {
    let mut parser = Parser::new();
    let grammar = match lang {
        Language::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        Language::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
    };
    parser
        .set_language(&grammar)
        .expect("failed to load grammar");
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

    #[test]
    fn parse_typescript_type_annotation() {
        let tree = parse_lang("const x: number = 1;", Language::TypeScript);
        let root = tree.root_node();
        assert!(!root.has_error());
    }

    #[test]
    fn parse_tsx_jsx_element() {
        let tree = parse_lang("const el = <div>hello</div>;", Language::Tsx);
        let root = tree.root_node();
        assert!(!root.has_error());
    }
}
