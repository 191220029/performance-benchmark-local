use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::statistics::compile_time_stat::CompileTimeResultSet;

pub fn write_tex(path: &Path, data: &CompileTimeResultSet) -> anyhow::Result<PathBuf> {
    let datas = collect_metrics(data);

    for (metric, data) in datas {
        let mut writer = BufWriter::new(File::create(
            path.join(format!("src-code-analyze-results-{metric}.tex")),
        )?);

        let metric = metric.replace("_", "\\_");
        let _ = writer.write(
            format!("\\begin{{table}}[]\n\
            \\caption{{src-code-analyze-result}}\n\
            \\adjustbox{{max width=\\textwidth}}{{\n\
            \\begin{{tabular}}{{lc|lc|lc}}\n\
            \\toprule\n\
            \\textbf{{Benchmark}} & \\textbf{{{metric}}} & \\textbf{{Benchmark}} & \\textbf{{{metric}}} & \\textbf{{Benchmark}} & \\textbf{{{metric}}}\\\\ \\midrule\n",)
            .as_bytes(),
        )?;

        let mut i = 0;
        let j = 3;
        let _ = writer.write(
            data.into_iter()
                .map(|(benchmark, stat)| {
                    let mut s = format!("\\texttt{{{benchmark}}} & {stat}");
                    if i == 0 {
                        i += 1;
                    } else if i < j {
                        s = "&".to_owned() + &s;
                        i += 1;
                    } else {
                        s = "\\\\\n".to_owned() + &s;
                        i = 1;
                    }
                    s
                })
                .collect::<String>()
                .as_bytes(),
        );

        if i < j {
            let _ = writer.write("\\\\".as_bytes());
        }

        let _ = writer.write(
            "\n\\bottomrule\n\
            \\end{tabular}}\n\
            \\end{table}\n"
                .as_bytes(),
        )?;
    }

    Ok(path.to_path_buf())
}

/// Transform data into `metric -> (benchmark -> stat)` form.
fn collect_metrics(data: &CompileTimeResultSet) -> HashMap<String, Vec<(String, f64)>> {
    let mut collected_data = HashMap::new();

    data.results.iter().for_each(|result| {
        result.result_vec.iter().for_each(|r| {
            r.stats.iter().for_each(|(metric, stat)| {
                if collected_data.get_mut(metric).is_none() {
                    collected_data.insert(metric.to_string(), vec![]);
                }
                collected_data
                    .get_mut(metric)
                    .unwrap()
                    .push((r.benchmark.clone(), stat));
            })
        })
    });

    collected_data.iter_mut().for_each(|(_, d)| {
        d.sort_by(|(a, _), (b, _)| a.to_lowercase().partial_cmp(&b.to_lowercase()).unwrap())
    });

    collected_data
}

#[cfg(test)]
mod test_tex_writer {
    use std::{
        fs::{remove_file, File},
        io::{BufReader, Read},
        path::PathBuf,
    };

    use super::write_tex;

    #[test]
    fn test_tex_writer() {
        let out_path = PathBuf::from("test/src_code_analyze/tex_writer");
        let data = PathBuf::from("test/src_code_analyze/tex_writer/data.json");
        let file = File::open(&data).unwrap();
        let data = serde_json::from_reader(BufReader::new(file)).unwrap();

        write_tex(&out_path, &data).unwrap();
        let verify_p = PathBuf::from("test/src_code_analyze/tex_writer/data-check.tex");

        let p = PathBuf::from(
            "test/src_code_analyze/tex_writer/src-code-analyze-results-slice_from_raw_parts.tex",
        );
        let mut buf = vec![];
        let f = File::open(&p).unwrap();
        let mut reader = BufReader::new(f);
        let _ = reader.read_to_end(&mut buf);

        let mut buf_verify = vec![];
        let f = File::open(&verify_p).unwrap();
        let mut reader = BufReader::new(f);
        let _ = reader.read_to_end(&mut buf_verify);

        assert_eq!(buf, buf_verify);

        remove_file(p).unwrap();
    }
}
