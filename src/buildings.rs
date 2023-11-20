use serde::Deserialize;

use super::Item;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildingRecord {
    pub slug: String,
    pub name: String,
    pub class_name: Item,
    pub description: String,
    pub categories: Vec<BuildingCategory>,
    pub build_menu_priority: usize,
    pub metadata: BuildingMetadata,
    pub size: BuildingSize,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Depends on the building
pub struct BuildingMetadata {
    pub power_consumption: Option<usize>,
    pub power_consumption_exponent: Option<f64>,
    pub manufacturing_speed: Option<usize>,
}


#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Building(String);

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum BuildingCategory {
    #[serde(rename = "SC_Manufacturers_C")]
    Manufacturers,

    #[serde(rename = "SC_Miners_C")]
    Miners,

    #[serde(rename = "SC_Smelters_C")]
    Smelters,

    #[serde(rename = "SC_OilProduction_C")]
    OilProduction,

    #[serde(rename = "SC_Generators_C")]
    Generators,
    
    #[serde(rename = "SC_PowerPoles_C")]
    PowerPoles,
    
    #[serde(rename = "SC_WallPoles_C")]
    WallPoles,

    #[serde(rename = "SC_ConveyorBelts_C")]
    ConveyorBelts,

    #[serde(rename = "SC_ConveyorConnections_C")]
    ConveyorConnections,

    #[serde(rename = "SC_ConveyorAttachments_C")]
    ConveyorAttachments,

    #[serde(rename = "SC_ConveyLift_C")]
    ConveyorLifts,

    #[serde(rename = "SC_ConverPole_C")]
    /// Conveyor Wall mount and floor hole
    ConveyorPoles,

    #[serde(rename = "SC_HyperTubes_C")]
    HyperTubes,
    
    #[serde(rename = "SC_Pipes_C")]
    Pipes,

    #[serde(rename = "SC_PipeSupport_C")]
    PipeSupports,

    #[serde(rename = "SC_Storage_C")]
    Storage,

    #[serde(rename = "SC_Vehicles_C")]
    /// Vehicle support buildings; i.e. Truck Station and Drone Station
    Vehicles,
    
    #[serde(rename = "SC_Trains_C")]
    Trains,

    
    #[serde(rename = "SC_JumpPads_C")]
    JumpPads,

    #[serde(rename = "SC_Towers_C")]
    /// For Lookout and radar towers
    Towers,

    #[serde(rename = "SC_Workstations_C")]
    Workstations,

    #[serde(rename = "SC_Special_C")]
    /// For HUB, Space Elevator, etc.
    Special,

    #[serde(rename = "SC_Signs_C")]
    Signs,

    #[serde(rename = "SC_Beams_C")]
    Beams,

    #[serde(rename = "SC_Catwalks_C")]
    Catwalks,

    #[serde(rename = "SC_Walkways_C")]
    Walkways,

    #[serde(rename = "SC_Attach_C")]
    /// Ex. Ladders, Railings
    DecorativeAttachments,

    #[serde(rename = "SC_Lights_C")]
    Lights,

    #[serde(rename = "SC_Frames_C")]
    Frames,
    
    #[serde(rename = "SC_Pillars_C")]
    Pillars,
    
    #[serde(rename = "SC_Floors_C")]
    Floors,

    #[serde(rename = "SC_Roofs_C")]
    Roofs,

    #[serde(rename = "SC_Walls_C")]
    Walls,

    #[serde(rename = "SC_DiagonalWalls_C")]
    DiagonalWalls,

    #[serde(rename = "SC_UpperDiagonalWalls_C")]
    UpperDiagonalWalls,

    #[serde(rename = "SC_AngularWalls_C")]
    AngularWals,

    #[serde(rename = "SC_Windows_C")]
    Windows,

    #[serde(rename = "SC_Doors_C")]
    Doors,

    #[serde(rename = "SC_Inverted_C")]
    /// For the inverted ramps
    InvertedRamps,

    #[serde(rename = "SC_QuatPipes_C")]
    /// Decorative Quarter Pipes
    QuarterPipes,

    #[serde(rename = "SC_Ramps_C")]
    Ramps,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Can be None or 0, depends on the building
pub struct BuildingSize {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub length: Option<f64>,
}
