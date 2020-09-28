use crate::Project;

#[derive(Clone, Default, Debug)]
pub struct Point(pub usize, pub usize);

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

#[derive(Clone, Debug)]
pub struct RenderCell {
    pub is_empty: bool,
    pub tile_id: usize,
    pub atlas_pos: Point,
}

#[derive(Debug)]
pub struct RenderGrid {
    pub tiles: Vec<RenderCell>,
    pub tile_size: Point,
    pub grid_size: Point,
}

impl RenderGrid {
    pub fn new(num_cells: usize, tile_size: Point, grid_size: Point) -> Self {
        RenderGrid {
            tiles: vec![
                RenderCell {
                    is_empty: true,
                    tile_id: 0,
                    atlas_pos: Point::default(),
                };
                num_cells
            ],
            tile_size,
            grid_size,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &RenderCell {
        let coord_id = grid_point_to_coord((x, y).into(), self.grid_size.0);
        &self.tiles[coord_id]
    }

    // pub fn iter_tiles(&self) -> std::slice::Iter<'i, std::slice::Iter<'i, &RenderCell>> {
    pub fn rows(&self) -> std::slice::Chunks<'_, RenderCell> {
        self.tiles.chunks(self.grid_size.0)
    }
}

pub trait ToRenderGrid {
    /// Creates a render grid with the layers merged down into a single layer.
    /// Excludes entity layers and intgrid layers but includes autotile and tile layers
    fn to_merged_render_grid(&self, level: usize) -> Result<RenderGrid, anyhow::Error>;

    // fn to_render_grid(&self, layer: isize) -> Result<Vec<RenderCell>, anyhow::Error>;
}

impl ToRenderGrid for Project {
    fn to_merged_render_grid(&self, level: usize) -> Result<RenderGrid, anyhow::Error> {
        if self.levels.len() < level {
            panic!("Level not found in the parsed map");
        }

        let level = &self.levels[level];

        // allocate the render grid
        let first_layer = &level.layer_instances[0];
        let cell_count = first_layer.grid_width * first_layer.grid_height;
        let mut grid = RenderGrid::new(
            cell_count,
            Point(16, 16),
            Point(first_layer.grid_width, first_layer.grid_height),
        );

        // iterate the layers. Run in reverse as we "draw up" the stack
        level.layer_instances.iter().rev().for_each(|layer| {
            layer.auto_tiles.iter().for_each(|rule| {
                rule.tiles.iter().for_each(|tile| {
                    grid.tiles[tile.coord_id].is_empty = false;
                    grid.tiles[tile.coord_id].tile_id = tile.tile_id;
                    grid.tiles[tile.coord_id].atlas_pos.0 = tile.tile_x;
                    grid.tiles[tile.coord_id].atlas_pos.1 = tile.tile_y;
                })
            });

            layer.grid_tiles.iter().for_each(|tile| {
                grid.tiles[tile.coord_id].is_empty = false;
                grid.tiles[tile.coord_id].tile_id = tile.tile_id;
                grid.tiles[tile.coord_id].atlas_pos.0 = tile.tile_x;
                grid.tiles[tile.coord_id].atlas_pos.1 = tile.tile_x;
            })
        });

        Ok(grid)
    }
}
