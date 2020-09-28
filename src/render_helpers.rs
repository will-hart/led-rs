use crate::Project;

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

pub struct RenderCell {
    pub world_pos: Point,
    pub atlast_pos: Point,
    pub atlas_size: Point,
}

pub trait ToRenderGrid {
    fn to_render_grid(&self) -> Vec<RenderCell>;
}

impl ToRenderGrid for Project {
    fn to_render_grid(&self) -> Vec<RenderCell> {
        vec![]
    }
}
