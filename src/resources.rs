use serde::Deserialize;

use super::{Item, Color};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRecord {
    pub item: Item,
    pub ping_color: Color,
    pub speed: usize,
}
