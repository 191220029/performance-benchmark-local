use tree_sitter::Tree;

use crate::ast_analyze::ops::{
    avg_args::avg_args, count_nodes::count_nodes, fn_avg_depth::fn_avg_depth,
    macro_count::macro_count,
};

pub fn ast_ops() -> Vec<Box<dyn Fn(&Tree) -> (String, f64)>> {
    vec![
        Box::new(count_nodes),
        Box::new(fn_avg_depth),
        Box::new(avg_args),
        Box::new(macro_count),
    ]
}
