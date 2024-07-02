use std::{
    collections::{HashMap, HashSet},
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

    let cached_dependency = &mut HashMap::default();

    for b in benchmarks {
        results.push(CompileTimeBenchResult {
            benchmark: b.name.clone(),
            iterations: 0,
            result_vec: vec![analyze_benchmark(
                &b,
                &dependency_dir,
                &ops,
                cached_dependency,
            )],
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
    ops: &Vec<Box<dyn Fn(&Tree, &[u8], &mut Stats, &String) -> (String, f64)>>,
    cached_dependency: &mut HashMap<PathBuf, Stats>,
) -> CompileTimeResult {
    println!(
        "analyzing benchmark {} {}",
        benchmark.name,
        benchmark.path.to_str().unwrap()
    );
    let stats = analyze_dir(
        &benchmark.path,
        &benchmark.name,
        dependency_dir,
        ops,
        &mut HashSet::default(),
        cached_dependency,
    )
    .unwrap();
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
    benchmark_name: &String,
    dependency_dir: &PathBuf,
    ops: &Vec<Box<dyn Fn(&Tree, &[u8], &mut Stats, &String) -> (String, f64)>>,
    analyzed_dependency: &mut HashSet<PathBuf>,
    cached_dependency: &mut HashMap<PathBuf, Stats>,
) -> anyhow::Result<Stats> {
    let mut stats = Stats::new();
    for entry in p.read_dir()? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            stats += analyze_dir(
                &entry.path(),
                benchmark_name,
                dependency_dir,
                ops,
                analyzed_dependency,
                cached_dependency,
            )
            .unwrap();
        } else if entry.file_type()?.is_file() {
            if entry.file_name().to_str().unwrap().ends_with(".rs") {
                let mut reader = BufReader::new(File::open(entry.path()).unwrap());
                let mut buf = vec![];
                if reader.read_to_end(&mut buf).is_err() {
                    continue;
                }

                let mut parser = Parser::new();
                if parser.set_language(&tree_sitter_rust::language()).is_err() {
                    continue;
                }
                let tree = parser.parse(&buf, None).unwrap();
                ops.iter().for_each(|op| {
                    let t = op(&tree, &buf, &mut stats, benchmark_name);
                    stats.add_or_insert(t.0, t.1)
                });
            } else if entry.file_name().to_str().unwrap().eq("Cargo.lock") {
                for d in read_dependencies(&entry.path()).unwrap() {
                    let path = &d.path(dependency_dir);
                    if path.exists() && !analyzed_dependency.contains(path) {
                        println!("  |---analyzing {}", d);
                        analyzed_dependency.insert(path.clone());
                        if let Some(stat) = cached_dependency.get(path) {
                            stats += stat.clone();
                        } else {
                            let stat = analyze_dir(
                                path,
                                benchmark_name,
                                dependency_dir,
                                ops,
                                analyzed_dependency,
                                cached_dependency,
                            )
                            .unwrap();
                            stats += stat.clone();
                            cached_dependency.insert(path.clone(), stat);
                        }
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
