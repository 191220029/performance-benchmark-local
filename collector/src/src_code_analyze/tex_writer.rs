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
            path.join(format!("src-code-analyze-results-{metric}.json")),
        )?);
        let _ = writer.write(
            format!("\\begin{{table}}[]\n\
            \\caption{{src-code-analyze-result}}\n\
            \\adjustbox{{max width=\\textwidth}}\
            \\toprule\n\
            \\begin{{tabular}}{{ll|ll|ll}}\n\
            \\toprule\n
            \\textbf{{Benchmark}} & \\textbf{metric} & \\textbf{{Benchmark}} & \\textbf{metric} & \\textbf{{Benchmark}} & \\textbf{metric}\\\\ \\midrule\n",)
            .as_bytes(),
        )?;

        let mut i = 0;
        let j = 3;
        let _ = writer.write(
            data.into_iter()
                .map(|(benchmark, stat)| {
                    let mut s = format!("\\texttt{benchmark} & {stat}");
                    if i < j {
                        s = s + "&";
                        i += 1;
                    } else {
                        s = s + "\\\\\n";
                        i = 0;
                    }
                    s
                })
                .collect::<String>()
                .as_bytes(),
        );

        let _ = writer.write(
            "\\bottomrule\
            \\end{tabular}}\
            \\end{table}"
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

    collected_data
        .iter_mut()
        .for_each(|(_, d)| d.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()));

    collected_data
}
