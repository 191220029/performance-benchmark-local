use tree_sitter::Tree;

use crate::execute::Stats;

pub fn file_number(_: &Tree, _: &[u8], _: &mut Stats, _: &String) -> (String, f64) {
    ("file_number".to_string(), 1.)
}
