mod cell;
mod board;
use crate::board::Board;

use macroquad::prelude::*;

#[macroquad::main("life")]
async fn main() {
    let mut primary_board = Board::<100, 100>::random();
    let mut update_toggle = false;
    
    loop {
        if is_key_pressed(KeyCode::F1) {
            primary_board = Board::random();
        }
        else if is_key_pressed(KeyCode::F4) {
            primary_board = Board::glider_gun()
        }
        if is_key_pressed(KeyCode::F3) {
           update_toggle = !update_toggle;
        }
        if is_key_pressed(KeyCode::F2) || update_toggle {
            primary_board.update();
        }

        primary_board.macroquad_draw();

        next_frame().await;
    }
}