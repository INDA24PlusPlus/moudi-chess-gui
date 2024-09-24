use crate::{HEIGHT, WIDTH};

pub const TILE_SIZE : i32 = 60;
pub const IMAGE_SIZE : i32 = 60;
pub const ATTACK_CIRCLE_RADIUS : i32 = 10;

const BOARD_SIZE : i32 = TILE_SIZE * 8;
const BOARD_OFFSET_X : i32 = (WIDTH - BOARD_SIZE) / 2;
const BOARD_OFFSET_Y : i32 = (HEIGHT - BOARD_SIZE) / 2;

pub fn board_coord_to_screen(x: i32, y: i32) -> (i32, i32) {
    (x * IMAGE_SIZE + BOARD_OFFSET_X, (7 - y) * IMAGE_SIZE + BOARD_OFFSET_Y)
}

pub fn screen_to_board_coord(x: i32, y: i32) -> (i32, i32) {
    ((x - BOARD_OFFSET_X) / IMAGE_SIZE, 7 - (y - BOARD_OFFSET_Y) / IMAGE_SIZE)
}
