use crate::tilesheet::SCALED_TILE_SIZE;

pub const LEVEL_ROWS: usize = 9;
pub const LEVEL_COLS: usize = 16;

pub const ROW_SCALED: f32 = LEVEL_ROWS as f32 * SCALED_TILE_SIZE as f32;
pub const COL_SCALED: f32 = LEVEL_COLS as f32 * SCALED_TILE_SIZE as f32;

pub const CELL_0_Y: f32 = (ROW_SCALED / 2.0) - SCALED_TILE_SIZE as f32 / 2.;
pub const CELL_0_X: f32 = -(COL_SCALED / 2.0) + SCALED_TILE_SIZE as f32 / 2.;
