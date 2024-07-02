use tree_sitter::Tree;

use crate::execute::Stats;
pub fn node_type(tree: &Tree, _: &[u8], _: &mut Stats, _: &String) -> (String, f64) {
    let mut cursor = tree.walk();
    let mut count = 0;

    loop {
        count += 1;

        // Try to go down to the first child
        if cursor.goto_first_child() {
            continue;
        }

        // If no children, try to go to the next sibling
        while !cursor.goto_next_sibling() {
            // If no next sibling, go up to the parent
            if !cursor.goto_parent() {
                // If no parent, we've reached the root again
                return ("node_count".to_string(), count as f64);
            }
        }
    }
}