use serde::Deserialize;

use super::Item;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemAmount {
    pub item: Item,
    pub amount: f64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}
