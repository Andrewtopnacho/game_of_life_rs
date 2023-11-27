mod cell;
mod board;
use crate::board::Board;

use macroquad::prelude::*;

#[macroquad::main("life")]
async fn main() {
    let mut board = Board::<100, 100>::random();
    let mut update_toggle = false;
    
    loop {
        if is_key_pressed(KeyCode::F1) {
            board = Board::random();
        }
        else if is_key_pressed(KeyCode::F4) {
            board = Board::glider_gun()
        }
        if is_key_pressed(KeyCode::F3) {
           update_toggle = !update_toggle;
        }
        if is_key_pressed(KeyCode::F2) || update_toggle {
            board.update();
        }

        board.macroquad_draw();

        next_frame().await;
    }
}