use anyhow::Result;
use std::fmt::Display;
use std::fs::{read_to_string, File};
use std::io::Write;

use crate::municipalities::data::PREFECTURES;
use crate::municipalities::serde_models::GeoJson;

#[allow(unused)]
pub fn output_sequences<T>(filename: &str, data: &[T]) -> Result<()>
where
    T: Display,
{
    let mut file = File::create(filename)?;
    for v in data {
        writeln!(file, "{}", v)?;
    }
    Ok(())
}

#[allow(unused)]
pub fn output_sequences_with_x<T>(filename: &str, x_data: &[f64], data: &[T]) -> Result<()>
where
    T: Display,
{
    let mut file = File::create(filename)?;
    for (x, y) in x_data.iter().zip(data) {
        writeln!(file, "{}  {}", x, y)?;
    }
    Ok(())
}

#[allow(unused)]
pub fn output_2d_sequences<T>(filename: &str, xy_data: &[[T; 2]]) -> Result<()>
where
    T: Display,
{
    let mut file = File::create(filename)?;
    for [x, y] in xy_data.iter() {
        writeln!(file, "{}  {}", x, y)?;
    }
    Ok(())
}

/// 都道府県名を指定して対応するファイルのデータを読み込む。
pub fn read_municipalities_boundary_data(prefecture_name: &str) -> Result<GeoJson> {
    let id = PREFECTURES
        .iter()
        .enumerate()
        .find(|(_, &pref)| pref == prefecture_name)
        .unwrap()
        .0;
    let filename = format!("N03-23_{}_230101.geojson", id);
    let json_content = read_to_string(filename)?;
    let json_obj = serde_json::from_str::<GeoJson>(&json_content)?;
    Ok(json_obj)
}
