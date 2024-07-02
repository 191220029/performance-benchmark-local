use tree_sitter::{Node, Tree};

use crate::execute::Stats;
pub fn fn_avg_depth(tree: &Tree, _: &[u8], _: &mut Stats, _: &String) -> (String, f64) {
    let mut cursor = tree.walk();
    let mut total_depth = 0.;
    let mut function_count = 0;

    loop {
        let node = cursor.node();
        if node.kind() == "function_item" {
            function_count += 1;
            total_depth += calculate_node_depth(&node);
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                let y = total_depth as f64 / function_count as f64;
                if y.is_nan() {
                    return ("fn_avg_depth".to_string(), 0.);
                }
                return ("fn_avg_depth".to_string(), y);
            }
        }
    }
}

fn calculate_node_depth(node: &Node) -> f64 {
    let mut cursor = node.walk();
    let mut depth_sum = 0;
    let mut node_cnt = 0;
    let mut depth = 1;

    loop {
        depth_sum += depth;
        node_cnt += 1;

        if cursor.goto_first_child() {
            depth += 1;
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return depth_sum as f64 / node_cnt as f64;
            }
            depth -= 1;
        }
    }
}
