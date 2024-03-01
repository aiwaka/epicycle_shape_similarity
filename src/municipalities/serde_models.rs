use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CrsProp {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crs {
    #[serde(rename = "type")]
    pub _type: String,
    pub properties: CrsProp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureGeometry {
    #[serde(rename = "type")]
    pub _type: String,
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoFeature {
    #[serde(rename = "type")]
    pub _type: String,
    pub properties: HashMap<String, Option<String>>,
    pub geometry: FeatureGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoJson {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub crs: Crs,
    pub features: Vec<GeoFeature>,
}
