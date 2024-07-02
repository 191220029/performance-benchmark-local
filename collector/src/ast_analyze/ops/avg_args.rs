use tree_sitter::{Node, Tree};

pub fn avg_args(tree: &Tree, _: &[u8]) -> (String, f64) {
    let mut cursor = tree.walk();
    let mut total_params = 0;
    let mut function_count = 0;

    loop {
        let node = cursor.node();
        if node.kind() == "function_item" {
            function_count += 1;
            total_params += count_function_parameters(&node);
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                if function_count == 0 {
                    return ("avg_args".to_string(), 0.);
                }
                return (
                    "avg_args".to_string(),
                    total_params as f64 / function_count as f64,
                );
            }
        }
    }
}

fn count_function_parameters(node: &Node) -> usize {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        if child.kind() == "parameters" {
            return child.named_child_count();
        }
    }
    0
}
