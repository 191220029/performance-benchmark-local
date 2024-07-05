use tree_sitter::Tree;

use crate::{
    ast_analyze::ops::{
        avg_args::avg_args, count_nodes::{avg_node_per_file, count_nodes}, field_count::{avg_field_reduce, field_struct_count}, file_number::file_number, fn_avg_depth::{fn_avg_depth, fn_depth}, io_calls::count_io_calls, macro_count::macro_count, parallel_calls::parallel_calls, struct_methods::{struct_avg_methods, struct_methods}
    },
    execute::Stats,
};

pub fn ast_ops() -> Vec<Box<dyn Fn(&Tree, &[u8], &mut Stats, &String) -> Vec<(String, f64)>>> {
    vec![
        Box::new(file_number),
        Box::new(count_nodes),
        // Box::new(fn_depth),
        // Box::new(avg_args),
        Box::new(macro_count),
        // Box::new(field_struct_count),
        Box::new(struct_methods),
        Box::new(parallel_calls),
        Box::new(count_io_calls),
    ]
}

pub fn reduce_ops() -> Vec<Box<dyn Fn(&mut Stats)>> {
    vec![
        // Box::new(avg_field_reduce),
        // Box::new(fn_avg_depth),
        // Box::new(struct_avg_methods),
        Box::new(avg_node_per_file),
    ]
}
