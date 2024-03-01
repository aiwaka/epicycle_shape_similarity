//! GeoJsonの座標情報を取り出しFFT可能な状態に置き換える

use std::collections::HashMap;

use anyhow::Result;
use rustfft::num_complex::Complex;

use crate::shapes::ShapePoints;

use super::serde_models::GeoFeature;

const ORDINANCE_DISIGNATED_CITIES: [&str; 20] = [
    "札幌市",
    "仙台市",
    "さいたま市",
    "千葉市",
    "横浜市",
    "川崎市",
    "相模原市",
    "静岡市",
    "浜松市",
    "名古屋市",
    "新潟市",
    "京都市",
    "大阪市",
    "堺市",
    "神戸市",
    "岡山市",
    "広島市",
    "北九州市",
    "福岡市",
    "熊本市",
];

pub fn geo_feature_props_to_array(
    props: &HashMap<String, Option<String>>,
) -> Result<[Option<String>; 7]> {
    let mut result: [Option<String>; 7] = [(); 7].map(|_| None);
    for (k, v) in props.iter() {
        let replaced_k = k.replace("N03_00", "");
        let num_k = replaced_k.parse::<usize>()?;
        result[num_k - 1] = v.clone();
    }
    Ok(result)
}

pub fn props_array_to_name(array: &[Option<String>; 7]) -> String {
    // N03_001: 都道府県名
    // N03_002: 支庁・振興局名（北海道のみ）
    // N03_003: 郡・政令都市名
    // N03_004: 市区町村名
    // 5, 6は欠番
    // N03_007: 行政区域コード

    array
        .iter()
        .take(6)
        .enumerate()
        .fold("".to_string(), |acc, (idx, v)| {
            format!(
                "{}{}",
                acc,
                match v {
                    // 3番目が政令指定都市の場合4番目に含まれているので除外
                    Some(v) if idx == 2 && ORDINANCE_DISIGNATED_CITIES.contains(&v.as_str()) =>
                        "".to_string(),
                    Some(v) => v.clone(),
                    None => "".to_string(),
                }
            )
        })
}

// /// Featureの列から、`properties`が指定された自治体コードに一致するものを取得する。
// /// 見つからない場合は`None`を返す。
// pub fn get_obj_has_specified_code(muni_code: String) -> Option<GeoFeature> {}

// /// 自治体コードから自治体名を返す（データを検索して取得する）。
// /// 見つからない場合は`None`を返す。
// pub fn get_muni_name_from_code(muni_code: String) -> Option<String> {}

// { "type": "Feature", "properties": { "N03_001": "京都府", "N03_002": null, "N03_003": "与謝郡", "N03_004": "伊根町", "N03_007": "26463" }, "geometry": { "type": "Polygon", "coordinates": [ [ [ 135.233368689513554, 35.769235694272027 ], [ 135.233357664724622, 35.769233640220477 ], [ 135.233333332667371, 35.769238467781236 ], [ 135.233269026644393, 35.769251279523075 ], [ 135.233243060519044, 35.769259225033352 ], [ 135.233229493346585, 35.769265171350753 ], [ 135.233219584616336, 35.769273134847481 ], [ 135.233218339954647, 35.769276197938325 ], [ 135.233221970517661, 35.76928919853782 ], [ 135.233235446858544, 35.769302252197349 ], [ 135.23327852078728, 35.769296505529496 ], [ 135.233333332667371, 35.769274721251577 ], [ 135.233371025052975, 35.769259748438799 ], [ 135.233374720367237, 35.7692537481621 ], [ 135.233376056759766, 35.769242666715854 ], [ 135.233368689513554, 35.769235694272027 ] ] ] } },

/// 自治体GISデータのFeatureデータから境界形状の点列を取得する。
/// 点の数を指定して間引くあるいは補間（中心補間）し、必ず最初の点と最後の点が一致するようにする。
pub fn convert_to_shape(geo_feature: &GeoFeature, result_points_num: usize) -> ShapePoints {
    // なぜか元データにおいて一回分Vec階層が多いので[0]を取得する必要がある
    let coordinates = &geo_feature.geometry.coordinates[0];
    let coord_data_num = coordinates.len();
    let results = if coord_data_num >= result_points_num {
        // 間引く場合
        let mut results = vec![];
        for idx in 0..result_points_num {
            let pick_idx = (coord_data_num - 1) * idx / (result_points_num - 1);
            let p_vec = &coordinates[pick_idx];
            let c = Complex::new(p_vec[0], p_vec[1]);
            results.push(c);
        }
        results
    } else {
        // 挿入する場合
        let points = coordinates
            .iter()
            .map(|p_vec| Complex::new(p_vec[0], p_vec[1]))
            .collect::<Vec<_>>();
        let points_num = coord_data_num;
        let mut results = vec![points[0]];
        let mut prev_idx_in_results = 0usize;
        let mut prev_p = points[0];
        for (idx, current_p) in points.iter().skip(1).enumerate() {
            // 今の添字番号が結果配列の何番にあたるか
            let idx_in_results = (result_points_num - 1) * (idx + 1) / (points_num - 1);
            let insert_num = idx_in_results - prev_idx_in_results;
            results.extend((1..=insert_num).map(|insert_i| {
                (current_p - prev_p) * insert_i as f64 / insert_num as f64 + prev_p
            }));
            prev_idx_in_results = idx_in_results;
            prev_p = *current_p;
        }
        results
    };
    results
}
