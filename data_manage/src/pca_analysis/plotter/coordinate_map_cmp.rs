use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use nalgebra::DVector;

use crate::pca_analysis::pca_data::PcaRawData;

use super::coordinate_map::{get_coordinate_2d, Coordinate};

/// `draw_coordinate_map_2d_cmp` plots the dataset,
/// shows the relationship between specific data
/// and principle_components against data_set_cmp.
///
/// parameter `principle_components` is a group of eigenvectors.
pub fn draw_coordinate_map_2d_cmp(
    principle_components: &Vec<Vec<f64>>,
    data_set: &dyn PcaRawData,
    data_set_cmp: &dyn PcaRawData,
    out_dir: &PathBuf,
) -> anyhow::Result<()> {
    // Step1. For each principle_component pair (u, v), do:
    //
    // Step2. Calculate the projection of each data on u and v,
    //        the projection value is the coordinate of the data.
    //
    // Step3. Generate coordinate map according to the coordinates.
    let feature_vectors: Vec<DVector<f64>> = principle_components
        .iter()
        .map(|u| DVector::from_vec(u.clone()))
        .collect();

    let mut iter_feature_vectors = feature_vectors.iter();
    let mut pc_u = 1;
    let mut pc_v = 2;

    feature_vectors.iter().for_each(|u| {
        iter_feature_vectors.next();
        let mut iter_v = iter_feature_vectors.clone();

        while let Some(v) = iter_v.next() {
            let coordinates: Vec<(Coordinate, String)> = data_set
                .iter_with_row_labels()
                .map(|(data, label)| (get_coordinate_2d(DVector::from_vec(data), u, v), label))
                .collect();

            let coordinates_cmp: Vec<(Coordinate, String)> = data_set_cmp
                .iter_with_row_labels()
                .map(|(data, label)| (get_coordinate_2d(DVector::from_vec(data), u, v), label))
                .collect();

            draw(coordinates, coordinates_cmp, pc_u, pc_v, out_dir).unwrap();
            pc_v += 1;
        }

        pc_u += 1;
        pc_v = pc_u + 1;
    });

    Ok(())
}

fn draw(
    coordinates: Vec<(Coordinate, String)>,
    coordinates_cmp: Vec<(Coordinate, String)>,
    pc_x: u32,
    pc_y: u32,
    out_dir: &PathBuf,
) -> anyhow::Result<()> {
    let mut scatter = Command::new("python");

    scatter
        .arg("src/pca_analysis/plotter/scatter_cmp.py")
        .arg(
            coordinates
                .into_iter()
                .map(|((x, y), label)| format!("{},{},{};", x.to_string(), y.to_string(), label))
                .collect::<String>(),
        )
        .arg(
            coordinates_cmp
                .into_iter()
                .map(|((x, y), label)| format!("{},{},{};", x.to_string(), y.to_string(), label))
                .collect::<String>(),
        )
        .arg(pc_x.to_string())
        .arg(pc_y.to_string())
        .arg(out_dir.join(format!("PC{}vsPC{}.png", pc_x, pc_y)));

    scatter.stdout(Stdio::inherit());
    scatter.stderr(Stdio::inherit());

    scatter.spawn()?.wait()?;

    Ok(())
}
