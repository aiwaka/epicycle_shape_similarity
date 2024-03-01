use std::collections::HashMap;

use maplit::hashmap;

#[test]
fn test_read_muni_data() {
    use crate::{
        io::read_municipalities_boundary_data, municipalities::utils::geo_feature_props_to_array,
    };

    let json_obj = read_municipalities_boundary_data().unwrap();
    // for feat in json_obj.features.iter() {
    //     println!("{:?}", feat.geometry.coordinates[0].len())
    // }
    let array = geo_feature_props_to_array(&json_obj.features[0].properties);
    println!("{:?}", array);
}

#[test]
/// 国土数値情報のGISデータの自治体境界のプロパティオブジェクトから自治体名が生成できることを検証する。
/// `raw_test_data`にできる限り書きやすい形で記述し、実際に用いられる`String`型に変換してからテストする。
fn test_get_muni_name() {
    use crate::municipalities::utils::{geo_feature_props_to_array, props_array_to_name};
    // ここにテストしたいpropertiesを書く
    let raw_test_data: Vec<(HashMap<&str, Option<&str>>, &str)> = vec![
        (
            hashmap![
                "N03_001" => Some("京都府"), "N03_002" => None, "N03_003" => Some("与謝郡"), "N03_004" => Some("伊根町"), "N03_007" => Some("26463")
            ],
            "京都府与謝郡伊根町",
        ),
        (
            hashmap![
                 "N03_001" => Some( "兵庫県"), "N03_002"=> None, "N03_003"=> Some("神戸市"), "N03_004"=> Some("神戸市東灘区"), "N03_007" => Some("28101")
            ],
            "兵庫県神戸市東灘区",
        ),
    ];

    let test_data = raw_test_data
        .into_iter()
        .map(|(hm, answer)| {
            (
                hm.into_iter()
                    .map(|(k, v)| (k.to_owned(), v.map(|v| v.to_owned())))
                    .collect::<HashMap<String, Option<String>>>(),
                answer.to_owned(),
            )
        })
        .collect::<Vec<(_, String)>>();

    for (props, answer) in test_data.iter() {
        let array = geo_feature_props_to_array(props);
        assert!(array.is_ok());
        let name = props_array_to_name(&array.unwrap());
        assert_eq!(&name, answer);
    }
}
