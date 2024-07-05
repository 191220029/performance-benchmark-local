use tree_sitter::Tree;

use crate::execute::Stats;

pub(super) const FILE_NUMBER: &str = "files";

pub fn file_number(_: &Tree, _: &[u8], _: &mut Stats, _: &String) -> Vec<(String, f64)> {
    vec![(FILE_NUMBER.to_string(), 1.)]
}
