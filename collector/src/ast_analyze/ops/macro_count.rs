extern crate tree_sitter;
extern crate tree_sitter_rust;

use tree_sitter::Tree;

pub fn macro_count(tree: &Tree) -> (String, f64) {
    let mut cursor = tree.walk();
    let mut macro_calls = 1;
    let mut macro_definitions = 1;

    loop {
        let node = cursor.node();

        // Count macro calls
        if node.kind() == "macro_invocation" {
            macro_calls += 1;
        }

        // Count macro definitions
        if node.kind() == "macro_definition" {
            macro_definitions += 1;
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return (
                    "macro".to_string(),
                    macro_calls as f64 / macro_definitions as f64,
                );
            }
        }
    }
}
