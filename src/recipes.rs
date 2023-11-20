use serde::Deserialize;

use super::{Item, ItemAmount, Building};

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Recipe(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecipeRecord {
    pub name: Recipe,
    pub slug: String,
    pub class_name: Item,
    pub alternate: bool,
    pub time: usize,
    pub manual_time_multiplier: f64,
    pub ingredients: Vec<ItemAmount>,
    pub for_building: bool,
    pub in_machine: bool,
    pub in_hand: bool,
    pub in_workshop: bool,
    pub is_variable_power: bool,
    pub products: Vec<ItemAmount>,
    pub produced_in: Vec<Building>
}
