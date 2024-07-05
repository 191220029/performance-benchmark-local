use tree_sitter::Tree;

use crate::execute::Stats;

use super::file_number::FILE_NUMBER;

const NODE_COUNT: &str = "node_count";
const AVG_NODE: &str = "nodes_per_file";

pub fn count_nodes(tree: &Tree, _: &[u8], _: &mut Stats, _: &String) -> Vec<(String, f64)> {
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
                return vec![(NODE_COUNT.to_string(), count as f64)];
            }
        }
    }
}

pub fn avg_node_per_file(stats: &mut Stats) {
    let files = stats.stats.remove(FILE_NUMBER).unwrap();
    let nodes = stats.stats.remove(NODE_COUNT).unwrap();

    stats.add_or_insert(AVG_NODE.to_string(), nodes / files)
}