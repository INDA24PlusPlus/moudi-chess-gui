use crate::{HEIGHT, WIDTH};

pub const TILE_SIZE : i32 = 60;
pub const IMAGE_SIZE : i32 = 60;
pub const ATTACK_CIRCLE_RADIUS : i32 = 10;

pub const BOARD_SIZE : i32 = TILE_SIZE * 8;
pub const BOARD_OFFSET_X : i32 = (WIDTH - BOARD_SIZE) / 2;
pub const BOARD_OFFSET_Y : i32 = (HEIGHT - BOARD_SIZE) / 2;

pub fn board_coord_to_screen(x: i32, y: i32) -> (i32, i32) {
    (x * IMAGE_SIZE + BOARD_OFFSET_X, (7 - y) * IMAGE_SIZE + BOARD_OFFSET_Y)
}

pub fn screen_to_board_coord(x: i32, y: i32) -> (i32, i32) {
    let diff_x = x - BOARD_OFFSET_X;
    let diff_y = y - BOARD_OFFSET_Y;
    if diff_x < 0 || diff_y < 0 { (-1, -1) }
    else { (diff_x / IMAGE_SIZE, 7 - diff_y / IMAGE_SIZE) }
}
