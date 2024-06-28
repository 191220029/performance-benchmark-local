use std::{
    collections::HashSet,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read},
    path::PathBuf,
};

use tree_sitter::{Parser, Tree};

use crate::{
    ast_analyze::parse::ast_ops,
    benchmark::{benchmark::Benchamrk, profile::Profile, scenario::Scenario},
    compile_time::discover_benchmark_suit,
    execute::Stats,
    src_code_analyze::{dependancy::read_dependencies, tex_writer::write_tex},
    statistics::compile_time_stat::{
        CompileTimeBenchResult, CompileTimeResult, CompileTimeResultSet,
    },
};

pub fn ast_code_analyze(
    bench_dir: PathBuf,
    dependency_dir: PathBuf,
    out_path: PathBuf,
) -> anyhow::Result<PathBuf> {
    assert!(dependency_dir.exists());
    let benchmarks = discover_benchmark_suit(&bench_dir)?;

    let ops = ast_ops();

    let mut results = vec![];

    for b in benchmarks {
        results.push(CompileTimeBenchResult {
            benchmark: b.name.clone(),
            iterations: 0,
            result_vec: vec![analyze_benchmark(&b, &dependency_dir, &ops)],
        });
    }

    let results = CompileTimeResultSet::new(0.to_string(), results);
    create_dir_all(&out_path)?;
    serde_json::to_writer(
        BufWriter::new(File::create(
            &out_path.join("src-code-analyze-results.json"),
        )?),
        &results,
    )?;

    write_tex(&out_path, &results)?;

    Ok(out_path)
}

fn analyze_benchmark(
    benchmark: &Benchamrk,
    dependency_dir: &PathBuf,
    ops: &Vec<Box<dyn Fn(&Tree) -> (String, f64)>>,
) -> CompileTimeResult {
    eprintln!(
        "analyzing benchmark {} {}",
        benchmark.name,
        benchmark.path.to_str().unwrap()
    );
    let stats = analyze_dir(&benchmark.path, dependency_dir, ops, &mut HashSet::new()).unwrap();
    CompileTimeResult::new(
        benchmark.name.clone(),
        0,
        Profile::Check,
        Scenario::Full,
        stats,
    )
}

fn analyze_dir(
    p: &PathBuf,
    dependency_dir: &PathBuf,
    ops: &Vec<Box<dyn Fn(&Tree) -> (String, f64)>>,
    analyzed_dependency: &mut HashSet<PathBuf>,
) -> anyhow::Result<Stats> {
    let mut stats = Stats::new();
    for entry in p.read_dir()? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            stats += analyze_dir(&entry.path(), dependency_dir, ops, analyzed_dependency)?;
        } else if entry.file_type()?.is_file() {
            if entry.file_name().to_str().unwrap().ends_with(".rs") {
                let mut reader = BufReader::new(File::open(entry.path())?);
                let mut buf = vec![];
                reader.read_to_end(&mut buf)?;

                let mut parser = Parser::new();
                parser
                    .set_language(&tree_sitter_rust::language())
                    .expect("Error loading Rust grammar");
                let tree = parser.parse(buf, None).unwrap();
                assert!(!tree.root_node().has_error());
                // if let Ok(buf) = String::from_utf8(buf) {
                ops.iter().for_each(|op| {
                    let t = op(&tree);
                    stats.add_or_insert(t.0, t.1)
                });
                // }
            } else if entry.file_name().to_str().unwrap().eq("Cargo.lock") {
                for d in read_dependencies(&entry.path())? {
                    let path = &d.path(dependency_dir);
                    if path.exists() && !analyzed_dependency.contains(path) {
                        println!("  |---analyzing {}", d);
                        analyzed_dependency.insert(path.clone());
                        stats += analyze_dir(path, dependency_dir, ops, analyzed_dependency)?;
                    }
                }
            }
        }
    }
    Ok(stats)
}

#[cfg(test)]
mod test_ast {
    use std::path::PathBuf;

    use super::ast_code_analyze;

    #[test]
    fn test_ast_parser() {
        let benchmark_dir = PathBuf::from("test/binary_size/benchmarks");
        let out_dir = PathBuf::from("test/ast_analyze/out");
        let p = ast_code_analyze(benchmark_dir.clone(), benchmark_dir, out_dir).unwrap();
        assert!(p.exists());
    }
}
