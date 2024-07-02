use std::{
    fs::{create_dir_all, File},
    io::BufReader,
    path::PathBuf,
};

use collector::mir_analyze::data::table_data::{sort, TableDatas};

use super::{
    pca_data::get_principle_components,
    plotter::{
        coordinate_map::draw_coordinate_map_2d, coordinate_map_cmp::draw_coordinate_map_2d_cmp,
    },
};

pub fn pca_entry(
    table_data_path: &PathBuf,
    table_data_cmp_path: Option<PathBuf>,
    out_dir: PathBuf,
    max_component_num: u32,
) -> anyhow::Result<PathBuf> {
    let table_data: TableDatas<String, String, f64> =
        serde_json::from_reader(BufReader::new(File::open(table_data_path)?))?;

    let pc = get_principle_components(&table_data, max_component_num);
    display_pc(&pc, &table_data);
    create_dir_all(&out_dir)?;

    if let Some(table_data_cmp_path) = table_data_cmp_path {
        let table_data_cmp: TableDatas<String, String, f64> =
            serde_json::from_reader(BufReader::new(File::open(table_data_cmp_path)?))?;
        draw_coordinate_map_2d_cmp(&pc, &table_data, &table_data_cmp, &out_dir)?;
    } else {
        draw_coordinate_map_2d(&pc, &table_data, &out_dir)?;
    }

    Ok(out_dir)
}

pub(super) fn display_pc(pcs: &Vec<Vec<f64>>, table_data: &TableDatas<String, String, f64>) {
    let labels = sort(table_data)
        .into_iter()
        .map(|(s, _)| s)
        .collect::<Vec<String>>()
        .join("\t");

    let mut k = 0;
    println!("=====================================================");
    println!("{}", labels);
    for pc in pcs {
        k += 1;
        println!(
            "PC{}: {}",
            k,
            pc.into_iter()
                .map(|p| format!("{:.2}", p))
                .collect::<Vec<String>>()
                .join("\t")
        );
    }
    println!("=====================================================");
}
