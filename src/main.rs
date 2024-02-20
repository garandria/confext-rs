use tree_sitter::{Parser, Language, Tree, Query};
use std::{fs, str};

fn main() {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_bash::language())
	.expect("Error loading Bash grammar");
    let code_bytes = fs::read("examples/x264-master/configure").unwrap();
    let tree = parser.parse(code_bytes.clone(), None).unwrap();
    let mut cursor = tree.walk();
    let mut stack = Vec::new();
    let mut indent = String::from("");
    stack.push(cursor.node());
    while !stack.is_empty() {
	let node = stack.pop().unwrap();
	let node_str = str::from_utf8(&code_bytes[node.start_byte()..node.end_byte()]).unwrap();
	println!("{}{}: {}", indent, node.kind(), node_str);
	while cursor.goto_next_sibling() {
	    stack.push(cursor.node())
	}
	cursor.reset(node);
	if cursor.goto_first_child() {
	    stack.push(cursor.node());
	    indent.push_str("  ");
	}else {
	    indent.truncate(2);
	}
    }
}
