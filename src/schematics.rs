use serde::Deserialize;

use super::{ItemAmount, Recipe, Item};

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Schematic(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchematicRecord {
    pub name: String,
    pub slug: String,
    pub class_name: Schematic,
    pub tier: usize,
    pub cost: Vec<ItemAmount>,
    pub unlock: Unlocks,
    /// Note that not all require schematics are listed as a schematic record.
    pub required_schematics: Vec<Schematic>,
    #[serde(rename = "type")]
    pub schematic_type: SchematicType,
    pub time: usize,
    pub alternate: bool,
    pub mam: bool,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Unlocks {
    pub inventory_slots: usize,
    pub recipes: Vec<Recipe>,
    pub scanner_resources:  Vec<Item>,
    pub give_items: Vec<ItemAmount>
}

#[derive(Deserialize, Debug)]
pub enum SchematicType {
    #[serde(rename = "EST_MAM")]
    Mam,
    
    #[serde(rename = "EST_HardDrive")]
    HardDrive,
    
    #[serde(rename = "EST_ResourceSink")]
    ResourceSink,
    
    #[serde(rename = "EST_Milestone")]
    Milestone,

    #[serde(rename = "EST_Alternate")]
    Alternate,

    #[serde(rename = "EST_Tutorial")]
    Tutorial,
}
