use tree_sitter::{Node, Tree};

pub fn struct_methods(tree: &Tree, _: &[u8]) -> (String, f64) {
    let mut cursor = tree.walk();
    let mut total_methods = 0;
    let mut struct_count = 0;

    loop {
        let node = cursor.node();

        if node.kind() == "struct_item" {
            struct_count += 1;
        } else if node.kind() == "impl_item" {
            total_methods += find_methods(&node);
        }

        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                if struct_count == 0 {
                    return ("struct_methods".to_string(), 0.);
                }

                return (
                    "struct_methods".to_string(),
                    total_methods as f64 / struct_count as f64,
                );
            }
        }
    }
}

fn find_methods(node: &Node) -> usize {
    let mut method_count = 0;
    let mut cursor = node.walk();

    loop {
        let node = cursor.node();

        if node.kind() == "function_item" {
            method_count += 1;
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    return method_count;
                }
            }
            continue;
        }
        if cursor.goto_first_child() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return method_count;
            }
        }
    }
}

#[cfg(test)]
mod test_struct_methods {
    use tree_sitter::Parser;

    use crate::ast_analyze::ops::struct_methods::struct_methods;

    #[test]
    fn test_struct_methods() {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::language())
            .expect("Error loading Rust grammar");

        let source_code = r#"
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    struct Circle {
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            3.14 * self.radius * self.radius
        }
    }
    "#;

        let tree = parser.parse(source_code, None).unwrap();
        let (_, struct_methods) = struct_methods(&tree, source_code.as_bytes());

        assert_eq!(struct_methods, 1.5);
    }
}
