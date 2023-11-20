use serde::Deserialize;

use super::Item;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Generator(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneratorRecord {
    pub class_name: Generator,
    pub fuel: Vec<Item>,
    pub fuels: Vec<Fuel>,
    pub power_production: usize,
    pub power_production_exponent: f64,
    pub water_to_power_ratio: f64,
}
            
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fuel {
    pub item: Item,
    pub supplemental_item: Option<Item>,
    pub byproduct: Option<Item>,
    pub byproduct_amount: Option<usize>,
}

