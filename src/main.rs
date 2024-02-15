use tree_sitter::{Parser, Language};

fn main() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_bash::language())
	.expect("Error loading Bash grammar");
}
