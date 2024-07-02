use tree_sitter::Tree;

use crate::ast_analyze::ops::{
    avg_args::avg_args, count_nodes::count_nodes, field_count::field_count,
    fn_avg_depth::fn_avg_depth, macro_count::macro_count, parallel_calls::parallel_calls,
    struct_methods::struct_methods,
};

pub fn ast_ops() -> Vec<Box<dyn Fn(&Tree, &[u8]) -> (String, f64)>> {
    vec![
        Box::new(count_nodes),
        Box::new(fn_avg_depth),
        Box::new(avg_args),
        Box::new(macro_count),
        Box::new(field_count),
        Box::new(struct_methods),
        Box::new(parallel_calls),
    ]
}
