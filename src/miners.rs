use serde::Deserialize;

use super::Item;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
/// Note that this is not the same as the Building struct, 
/// the miner records use unique naming
pub struct MinerBuilding(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinerRecord {
    pub class_name: MinerBuilding,
    pub allowed_resources: Vec<Item>,
    pub items_per_cycle: usize,
    pub extract_cycle_time: f64,
    pub allow_liquids: bool,
    pub allow_solids: bool,
}