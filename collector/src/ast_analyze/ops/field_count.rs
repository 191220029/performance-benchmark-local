use tree_sitter::{Node, Tree};

pub fn field_count(tree: &Tree, _: &[u8]) -> (String, f64) {
    let mut cursor = tree.walk();
    let mut fields = 0;
    let mut type_count = 0;

    loop {
        let node = cursor.node();

        // Count struct and enum members
        if node.kind() == "struct_item" || node.kind() == "enum_item" {
            type_count += 1;
            fields += count_type_members(&node);
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                if type_count == 0 {
                    return ("fields".to_string(), 0.);
                }
                return ("fields".to_string(), fields as f64 / type_count as f64);
            }
        }
    }
}

fn count_type_members(node: &Node) -> usize {
    let mut cursor = node.walk();
    let mut member_count = 0;

    loop {
        let child = cursor.node();
        if child.kind() == "field_declaration" || child.kind() == "enum_variant" {
            member_count += 1;
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return member_count;
            }
        }
    }
}
