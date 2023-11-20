use serde::Deserialize;

use super::Color;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Item(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemRecord {
    pub name: String,
    pub slug: String,
    pub class_name: Item,
    pub sink_points: Option<usize>,
    pub description: String,
    pub stack_size: usize,
    pub energy_value: f64,
    pub radioactive_decay: f64,
    pub liquid: bool,
    pub fluid_color: Color
}