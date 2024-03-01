use std::f64::consts::TAU;

use regex::Regex;
use rustfft::num_complex::Complex;

use crate::{
    io::read_municipalities_boundary_data,
    municipalities::utils::{convert_to_shape, geo_feature_props_to_name, normalize_shape},
};

pub type ShapePoints = Vec<Complex<f64>>;
const NUM_SAMPLES: usize = 128;

pub fn simple_circle() -> ShapePoints {
    (0..NUM_SAMPLES)
        .map(|idx| 200.0 * Complex::cis(TAU * idx as f64 / (NUM_SAMPLES - 1) as f64))
        .collect()
}

pub fn rectangle() -> ShapePoints {
    use std::f64::consts::FRAC_PI_2;
    let mut points = Vec::<Complex<f64>>::new();
    for idx in 0..NUM_SAMPLES {
        let phase = TAU * idx as f64 / (NUM_SAMPLES - 1) as f64;
        let re = phase.cos();
        let im = phase.sin();

        let c = if re > im {
            if re > -im {
                Complex::new(1.0, phase.tan())
            } else {
                Complex::new((FRAC_PI_2 + phase).tan(), -1.0)
            }
        } else if re > -im {
            Complex::new((FRAC_PI_2 - phase).tan(), 1.0)
        } else {
            Complex::new(-1.0, -phase.tan())
        } * 100.0;
        points.push(c)
    }
    points
}

pub fn flower() -> ShapePoints {
    let mut points = Vec::<Complex<f64>>::new();
    for idx in 0..NUM_SAMPLES {
        let phase = TAU * idx as f64 / (NUM_SAMPLES - 1) as f64;
        let c = 150.0 * Complex::cis(phase) + 50.0 * Complex::cis(phase * 5.0);
        points.push(c)
    }
    points
}

/// 自治体名をGISデータ内の完全名で与え、境界形状点列データを取得する。
pub fn municipality_shape(muni_name: &str, result_point_num: usize) -> ShapePoints {
    // 都道府県名と、そのファイル内の欲しい境界データ番号を指定
    // 向日市
    // let (prefecture_name, feature_id) = ("京都府", 354);
    // 城陽市
    // let (prefecture_name, feature_id) = ("京都府", 353);
    // 姫路市
    // let (prefecture_name, feature_id) = ("兵庫県", 194);

    // NOTE: 都道府県名一覧データがあるのでそちらを使っても良さそう
    let re = Regex::new(r"([^\x00-\x7F]{2,3}県|..府|東京都|北海道)").unwrap();
    let prefecture_name = re.captures(muni_name).unwrap().get(0).unwrap().as_str();

    let json_data = read_municipalities_boundary_data(prefecture_name).unwrap();
    // 指定された自治体を示すpropertiesを持つfeatureの中で要素数が最も多いものを探す
    let geo_feature = json_data
        .features
        .iter()
        .filter(|feat| geo_feature_props_to_name(&feat.properties) == muni_name)
        .max_by_key(|x| x.geometry.coordinates.len())
        .unwrap();
    let shape = convert_to_shape(geo_feature, result_point_num);
    normalize_shape(shape)
}
