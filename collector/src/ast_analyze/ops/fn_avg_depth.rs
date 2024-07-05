use tree_sitter::{Node, Tree};

use crate::execute::Stats;

const FN_LABEL: &str = "fn_count";
const DEPTH_LABEL: &str = "fn_depth";
const AVG_LABEL: &str = "fn_avg_depth";

pub fn fn_depth(tree: &Tree, _: &[u8], _: &mut Stats, _: &String) -> Vec<(String, f64)> {
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
                return vec![
                    (FN_LABEL.to_string(), function_count as f64),
                    (DEPTH_LABEL.to_string(), total_depth as f64),
                ];
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

pub fn fn_avg_depth(stats: &mut Stats) {
    let fns = stats.stats.remove(FN_LABEL).unwrap();
    let depths = stats.stats.remove(DEPTH_LABEL).unwrap();

    if fns == 0. {
        stats.add_or_insert(AVG_LABEL.to_string(), 0.);
    } else {
        stats.add_or_insert(AVG_LABEL.to_string(), fns / depths);
    }
}
