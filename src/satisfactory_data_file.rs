use std::collections::HashMap;

use serde::Deserialize;

use super::*;


#[derive(Deserialize)]
pub struct SatisfactoryData {
    pub recipes: HashMap<Recipe, RecipeRecord>,
    pub items: HashMap<Item, ItemRecord>,
    pub schematics: HashMap<Schematic, SchematicRecord>,
    pub generators: HashMap<Generator, GeneratorRecord>,
    pub resources: HashMap<Item, ResourceRecord>,
    pub miners: HashMap<MinerBuilding, MinerRecord>,
    pub buildings: HashMap<Building, BuildingRecord>
}
