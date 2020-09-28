/// A basic parser for LEd files
/// See https://deepnight.net/docs/led/json/ for the format
/// Configured for version 0.2.1 of the editor, version 1 JSON files
use anyhow;
use serde;
use serde::Deserialize;
use serde_json::from_str;

pub mod render_helpers;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Project {
    /// Contains the full path to the project JSON, as provided to the macro (using slashes)
    pub project_file_path: Option<String>,

    /// Contains the directory of the project JSON (using slashes, no trailing slash)
    pub project_dir: Option<String>,

    /// Project name
    pub name: String,

    /// Project background color as hex (e.g. #FFFFFF)
    pub bg_color: String,

    pub json_version: String,
    pub default_pivot_x: f64,
    pub default_pivot_y: f64,

    pub levels: Vec<Level>,
    // pub defs: Definitions, omitted, use the fields on the level layers
}

impl Project {
    pub fn parse_json(json: String) -> Result<Self, anyhow::Error> {
        Ok(from_str(&json)?)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Level {
    pub identifier: String,
    pub px_wid: isize,
    pub px_hei: isize,

    pub layer_instances: Vec<LayerInstance>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LayerInstance {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifer: String,

    #[serde(rename(deserialize = "__type"))]
    pub layer_type: String,

    #[serde(rename(deserialize = "__cWid"))]
    pub grid_width: usize,

    #[serde(rename(deserialize = "__cHei"))]
    pub grid_height: usize,

    pub level_id: usize,
    pub layer_def_uid: usize,
    pub px_offset_x: isize,
    pub px_offset_y: isize,
    pub seed: usize,

    pub int_grid: Vec<IntGridCoordinate>,
    pub auto_tiles: Vec<AutoTileRule>,
    pub grid_tiles: Vec<GridTile>,
    pub entity_instances: Vec<EntityInstance>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct IntGridCoordinate {
    pub coord_id: usize,
    pub v: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AutoTileRule {
    pub rule_id: usize,
    pub tiles: Vec<GridTile>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GridTile {
    pub coord_id: usize,
    pub tile_id: usize,
    pub flips: Option<usize>,
    #[serde(rename(deserialize = "__tileX"))]
    pub tile_x: usize,
    #[serde(rename(deserialize = "__tileY"))]
    pub tile_y: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct EntityInstance {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__cx"))]
    pub cx: isize,

    #[serde(rename(deserialize = "__cy"))]
    pub cy: isize,

    pub def_uid: usize,
    pub x: isize,
    pub y: isize,

    pub field_instances: Vec<FieldInstanceValue>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "__type")]
pub enum FieldInstanceValue {
    Int(ScalarField<isize>),
    Float(ScalarField<f32>),
    Bool(ScalarField<bool>),
    String(ScalarField<String>),
    Color(ScalarField<String>),

    #[serde(rename = "Array<Int>")]
    IntArray(ArrayField<isize>),
    #[serde(rename = "Array<Float>")]
    FloatArray(ArrayField<f32>),
    #[serde(rename = "Array<Bool>")]
    BoolArray(ArrayField<bool>),
    #[serde(rename = "Array<String>")]
    StringArray(ArrayField<String>),
    #[serde(rename = "Array<Color>")]
    ColorArray(ArrayField<String>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ScalarField<T> {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__value"))]
    pub value: T,

    pub def_uid: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ArrayField<T> {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__value"))]
    pub value: Vec<T>,

    pub def_uid: usize,
}
