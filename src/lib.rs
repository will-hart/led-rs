/// A basic parser for LEd files
/// See https://deepnight.net/docs/led/json/ for the format
/// Configured for version 0.2.1 of the editor, version 1 JSON files
use anyhow;
use serde;
use serde::Deserialize;
use serde_json::from_str;

pub struct Point(usize, usize);

impl From<(usize, usize)> for Point {
    fn from(tup: (usize, usize)) -> Self {
        Point(tup.0, tup.1)
    }
}

/// Converts from a grid (x, y) point to a coordinate ID
pub fn grid_point_to_coord(point: Point, grid_width: usize) -> usize {
    point.0 + point.1 * grid_width
}

/// Converts from a coordinate ID to a grid (x, y) point
pub fn grid_coord_to_point(coord: usize, grid_width: usize) -> Point {
    let grid_y = coord / grid_width;
    Point(coord - grid_y * grid_width, coord / grid_width)
}

/// Converts from an atlast sprite ID to a pixel position in the atlas
pub fn tile_id_to_atlas_pixel(
    tile_id: usize,
    atlas_width: usize,
    grid_width: usize,
    padding: usize,
    spacing: usize,
) -> Point {
    let grid_x = tile_id - atlas_width * (tile_id / atlas_width);
    let grid_y = tile_id / atlas_width;

    Point(
        padding + grid_x * (grid_width + spacing),
        padding + grid_y * (grid_width + spacing),
    )
}

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
    Int(IntField),
    Float(FloatField),
    Bool(BoolField),
    String(StringField),
    Color(StringField),
    Array(StringArrayField),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct IntField {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__value"))]
    pub value: isize,

    pub def_uid: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FloatField {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__value"))]
    pub value: f64,

    pub def_uid: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BoolField {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__value"))]
    pub value: bool,

    pub def_uid: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct StringField {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__value"))]
    pub value: String,

    pub def_uid: usize,
} // include enum, colour

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct StringArrayField {
    #[serde(rename(deserialize = "__identifier"))]
    pub identifier: String,

    #[serde(rename(deserialize = "__value"))]
    pub value: Vec<String>,

    pub def_uid: usize,
}
