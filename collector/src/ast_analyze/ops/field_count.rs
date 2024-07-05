use tree_sitter::{Node, Tree};

use crate::execute::Stats;

const FIELD_LABEL: &str = "fields";
const STRUCT_LABEL: &str = "structs";
const AVG_FIELD_LABEL: &str = "avg_fields";

// count number of fields
pub fn field_struct_count(tree: &Tree, _: &[u8], _: &mut Stats, _: &String) -> Vec<(String, f64)> {
    let mut cursor = tree.walk();
    let mut fields = 0;
    let mut struct_count = 0;

    loop {
        let node = cursor.node();

        // Count struct and enum members
        if node.kind() == "struct_item" || node.kind() == "enum_item" {
            struct_count += 1;
            fields += count_type_members(&node);
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return vec![
                    (FIELD_LABEL.to_string(), fields as f64),
                    (STRUCT_LABEL.to_string(), struct_count as f64),
                ];
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

pub fn avg_field_reduce(stats: &mut Stats) {
    let structs = stats.stats.remove(STRUCT_LABEL).unwrap();
    let fields = stats.stats.remove(FIELD_LABEL).unwrap();

    if structs == 0. {
        stats.add_or_insert(AVG_FIELD_LABEL.to_string(), 0.);
    } else {
        stats.add_or_insert(AVG_FIELD_LABEL.to_string(), fields / structs);
    }
}
