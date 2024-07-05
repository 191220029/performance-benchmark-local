use std::collections::HashSet;

use tree_sitter::Tree;

use crate::execute::Stats;

const IO_LABEL: &str = "io_calls";

fn is_io_related(text: &str, pool: &HashSet<&str>) -> bool {
    pool.iter().any(|c| text.contains(c))
}

pub fn count_io_calls(tree: &Tree, src: &[u8], _: &mut Stats, _: &String) -> Vec<(String, f64)> {
    let mut cursor = tree.walk();
    let mut io_calls = 0;

    // Define a set of I/O related function and macro names
    let mut pool: HashSet<&str> = [
        "std::fs::File",
        "std::io::Write",
        "println!",
        "print!",
        "eprintln!",
        "eprint!",
        "read_to_string",
        "read_to_end",
        "write_all",
        "io",
    ]
    .iter()
    .cloned()
    .collect();

    loop {
        let node = cursor.node();

        if node.kind() == "use_declaration" {
            let mut walker: tree_sitter::TreeCursor = node.walk();
            node.children(&mut walker).for_each(|c| {
                if c.kind() == "scoped_identifier" {
                    let mut walker = c.walk();
                    let scoped_identifier = c.utf8_text(src).unwrap().to_string();
                    if pool.iter().any(|s| scoped_identifier.contains(s)) {
                        pool.insert(
                            c.children(&mut walker)
                                .last()
                                .unwrap()
                                .utf8_text(src)
                                .unwrap(),
                        );
                    }
                }
            });
        } else if node.kind() == "call_expression" || node.kind() == "macro_invocation" {

            if let Ok(call_text) = node.utf8_text(src) {
                if is_io_related(call_text, &pool) {
                    io_calls += 1;
                }
            }
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return vec![(IO_LABEL.to_string(), io_calls as f64)];
            }
        }
    }
}

#[cfg(test)]
mod test_io_calls {
    use tree_sitter::Parser;

    use crate::{ast_analyze::ops::io_calls::count_io_calls, execute::Stats};

    #[test]
    fn test_io_calls() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::language())
            .expect("Error loading Rust grammar");

        let source_code = r#"
        use std::fs::File;
        use std::io::{self, Read, Write};
    
        fn main() {
            let mut file = File::open("foo.txt").expect("Unable to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read file");
    
            println!("{}", contents);
    
            io::stdout().write_all(b"Hello, world!").unwrap();
        }
        "#;

        let tree = parser.parse(source_code, None).unwrap();

        let io_calls = count_io_calls(
            &tree,
            source_code.as_bytes(),
            &mut Stats::default(),
            &String::default(),
        );

        assert_eq!(8., io_calls.first().unwrap().1);
    }
}
