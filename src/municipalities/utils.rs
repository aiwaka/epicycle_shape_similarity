//! GeoJsonの座標情報を取り出しFFT可能な状態に置き換える

use std::collections::HashMap;

use anyhow::Result;

use crate::shapes::ShapePoints;

use super::serde_models::GeoJson;

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

// { "type": "Feature", "properties": { "N03_001": "京都府", "N03_002": null, "N03_003": "与謝郡", "N03_004": "伊根町", "N03_007": "26463" }, "geometry": { "type": "Polygon", "coordinates": [ [ [ 135.233368689513554, 35.769235694272027 ], [ 135.233357664724622, 35.769233640220477 ], [ 135.233333332667371, 35.769238467781236 ], [ 135.233269026644393, 35.769251279523075 ], [ 135.233243060519044, 35.769259225033352 ], [ 135.233229493346585, 35.769265171350753 ], [ 135.233219584616336, 35.769273134847481 ], [ 135.233218339954647, 35.769276197938325 ], [ 135.233221970517661, 35.76928919853782 ], [ 135.233235446858544, 35.769302252197349 ], [ 135.23327852078728, 35.769296505529496 ], [ 135.233333332667371, 35.769274721251577 ], [ 135.233371025052975, 35.769259748438799 ], [ 135.233374720367237, 35.7692537481621 ], [ 135.233376056759766, 35.769242666715854 ], [ 135.233368689513554, 35.769235694272027 ] ] ] } },

// pub fn convert_to_shape(geo: &GeoJson, max_points: Option<usize>) -> ShapePoints {}
