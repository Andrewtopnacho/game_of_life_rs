mod cell;
mod board;
use crate::board::Board;

use macroquad::prelude::*;

#[macroquad::main("life")]
async fn main() {
    let mut board = Board::<100, 100>::random();
    let mut play = false;
    
    loop {
        if is_key_pressed(KeyCode::F1) {
            board = Board::random();
        }
        else if is_key_pressed(KeyCode::F4) {
            board = Board::glider_gun()
        }
        if is_key_pressed(KeyCode::F3) {
           play = !play;
        }
        if is_key_pressed(KeyCode::F2) || play {
            board.update();
        }

        board.macroquad_draw();

        next_frame().await;
    }
}