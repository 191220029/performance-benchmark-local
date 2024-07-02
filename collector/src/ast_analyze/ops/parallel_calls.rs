use std::collections::HashSet;

use tree_sitter::{Node, Tree};

use crate::execute::Stats;
// List of keywords or function names related to concurrency or parallelism
const CONCURRENCY_KEYWORDS: &[&str] = &["std::thread", "tokio", "rayon", "async", "await"];

pub fn parallel_calls(tree: &Tree, src_code: &[u8], _: &mut Stats, _: &String) -> (String, f64) {
    let mut cursor = tree.walk();
    let mut concurrency_calls = 0;
    let mut pool: HashSet<String> =
        HashSet::from_iter(CONCURRENCY_KEYWORDS.into_iter().map(|s| s.to_string()));

    loop {
        let node = cursor.node();

        if node.kind() == "use_declaration" {
            let mut walker: tree_sitter::TreeCursor = node.walk();
            node.children(&mut walker).for_each(|c| {
                if c.kind() == "scoped_identifier" {
                    let mut walker = c.walk();
                    let scoped_identifier = c.utf8_text(src_code).unwrap().to_string();
                    if pool.iter().any(|s| scoped_identifier.contains(s.as_str())) {
                        pool.insert(
                            c.children(&mut walker)
                                .last()
                                .unwrap()
                                .utf8_text(src_code)
                                .unwrap()
                                .to_string(),
                        );
                    }
                }
            });
        } else if node.kind() == "async_block" {
            concurrency_calls += 1;
        } else if (node.kind() == "call_expression" || node.kind() == "macro_invocation")
            && is_concurrency_related(
                &node.utf8_text(src_code).unwrap().to_string().as_str(),
                &pool,
            )
        {
            concurrency_calls += find_parallel_call_in_call_macro(&node, src_code, &pool);
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return ("parallel_calls".to_string(), concurrency_calls as f64);
            }
        }
    }
}

fn find_parallel_call_in_call_macro(node: &Node, src: &[u8], pool: &HashSet<String>) -> usize {
    let mut cursor = node.walk();
    let mut concurrency_calls = 0;

    loop {
        let node = cursor.node();

        if node.kind() == "scoped_identifier" || node.kind() == "identifier" {
            if is_concurrency_related(node.utf8_text(src).unwrap().to_string().as_str(), pool) {
                concurrency_calls += 1;

                while !cursor.goto_next_sibling() {
                    if !cursor.goto_parent() {
                        return concurrency_calls;
                    }
                }
            }
        }
        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return concurrency_calls;
            }
        }
    }
}

fn is_concurrency_related(code: &str, pool: &HashSet<String>) -> bool {
    pool.iter().any(|kw| code.contains(kw.as_str()))
}

#[cfg(test)]
mod test_struct_methods {
    use tree_sitter::Parser;

    use crate::{ast_analyze::ops::parallel_calls::parallel_calls, execute::Stats};

    #[test]
    fn test_struct_methods() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::language())
            .expect("Error loading Rust grammar");

        let source_code = r#"
            use std::thread;
        
            fn main() {
                thread::spawn(|| {
                    println!("Hello from a thread!");
                });
        
                tokio::spawn(async {
                    println!("Hello from a tokio task!");
                });
        
                async {
                    println!("Hello from an async block!");
                };
            }
            "#;

        let tree = parser.parse(source_code, None).unwrap();
        let (_, parallel_calls) = parallel_calls(
            &tree,
            source_code.as_bytes(),
            &mut Stats::default(),
            &String::default(),
        );

        assert_eq!(parallel_calls, 4.);
    }
}
