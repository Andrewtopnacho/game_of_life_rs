use crate::cell::Cell;

#[derive(Clone, Copy)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Cell; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    // constructors
    pub fn random() -> Board<WIDTH, HEIGHT> {
        use rand::random;

        let mut random_board = Board::default();
        for row in &mut random_board.cells {
            for cell in row {
                if random::<usize>() % 5 == 0 {
                    cell.change_state();
                }
            }
        }
        random_board
    }
    pub fn glider_gun() -> Board<WIDTH, HEIGHT> {
        use Cell::{Alive as O, Dead as X};

        let glider_gun = [
    //        0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 19 30 31 32 33 34 35 36 37
    /*0*/   [ X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*1*/   [ X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, O, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*2*/   [ X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, O, X, O, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*3*/   [ X, X, X, X, X, X, X, X, X, X, X, X, X, O, O, X, X, X, X, X, X, O, O, X, X, X, X, X, X, X, X, X, X, X, X, O, O, X, ],
    /*4*/   [ X, X, X, X, X, X, X, X, X, X, X, X, O, X, X, X, O, X, X, X, X, O, O, X, X, X, X, X, X, X, X, X, X, X, X, O, O, X, ],
    /*5*/   [ X, O, O, X, X, X, X, X, X, X, X, O, X, X, X, X, X, O, X, X, X, O, O, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*6*/   [ X, O, O, X, X, X, X, X, X, X, X, O, X, X, X, O, X, O, O, X, X, X, X, O, X, O, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*7*/   [ X, X, X, X, X, X, X, X, X, X, X, O, X, X, X, X, X, O, X, X, X, X, X, X, X, O, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*8*/   [ X, X, X, X, X, X, X, X, X, X, X, X, O, X, X, X, O, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*9*/   [ X, X, X, X, X, X, X, X, X, X, X, X, X, O, O, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, ],
    /*10*/  [ X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, ],
        ];   

        let mut cells = [[Cell::Dead; WIDTH]; HEIGHT];

        for (row_index, row) in cells.iter_mut().enumerate() {
            for (column_index, cell) in row.iter_mut().enumerate() {

                let glider_gun_cell = match glider_gun.get(column_index) {
                    Some(glider_gun_row) => match glider_gun_row.get(row_index) {
                        Some(glider_gun_cell) => glider_gun_cell,
                        None => continue,
                    },
                    None => continue,
                };

                *cell = *glider_gun_cell;
            }
        }
        
        Board { cells }
    }
    
    // actions
    pub fn update(&mut self) {
        let buffer = self.clone();
    
        for (row_index, row) in self.cells.iter_mut().enumerate() {
            for (column_index, current_cell) in row.iter_mut().enumerate() {
    
                let neighbor_count = buffer.get_neighbor_count(row_index, column_index);
                current_cell.update(neighbor_count);
            }
        }
    }
    pub fn macroquad_draw(&self) {
        use macroquad::prelude::*;
    
        const CELL_SIZE: f32 = 20.0;
        const BORDER_SIZE: f32 = 1.0;
        
        const BORDER_COLOR: Color = BLACK;
        const ALIVE_COLOR: Color = YELLOW;
        const DEAD_COLOR: Color = LIGHTGRAY;
        
    
        for (row_index, row) in self.cells.iter().enumerate() {
            for (column_index, &current_cell) in row.iter().enumerate() {
                
                let current_cell_position = Vec2::new(row_index as f32, column_index as f32) * CELL_SIZE;
                let current_cell_color = 
                    if current_cell == Cell::Alive {
                        ALIVE_COLOR
                    } else {
                        DEAD_COLOR
                    };
    
                draw_rectangle(
                    current_cell_position.x,
                    current_cell_position.y,
                    CELL_SIZE,
                    CELL_SIZE,
                    BORDER_COLOR,
                );
    
                draw_rectangle(
                    current_cell_position.x + BORDER_SIZE,
                    current_cell_position.y + BORDER_SIZE,
                    CELL_SIZE - BORDER_SIZE,
                    CELL_SIZE - BORDER_SIZE,
                    current_cell_color 
                );
            }
        }
    }
    
    // helper functions
    fn get_neighbor_count(&self, row_index: usize, column_index: usize) -> usize {
        
        let top_left = if row_index != 0 && column_index != 0 {
            match self.cells.get(row_index - 1) {
            Some(row) => match row.get(column_index - 1) {
                Some(cell) => cell,
                None => &Cell::Dead,
            },
            None => &Cell::Dead,
            }
        } else { &Cell::Dead };  
        let top = if row_index != 0 {
            match self.cells.get(row_index - 1) {
                Some(row) => &row[column_index],
                None => &Cell::Dead,                   
            }
        } else { &Cell::Dead };
        let top_right = if row_index != 0 {
            match self.cells.get(row_index - 1) {
                Some(row) => match row.get(column_index + 1) {
                    Some(cell) => cell,
                    None => &Cell::Dead,
                }
                None => &Cell::Dead,
            }
        } else { &Cell::Dead };
        let left = if column_index != 0 {
            match self.cells[row_index].get(column_index - 1) {
                Some(cell) => cell,
                None => &Cell::Dead,
            }
        } else { &Cell::Dead };
        let right = match self.cells[row_index].get(column_index + 1) {
            Some(cell) => cell,
            None => &Cell::Dead,
        };
        let bottom_left = match self.cells.get(row_index + 1) {
            Some(row) => if column_index != 0 {
                match row.get(column_index - 1) {
                    Some(cell) => cell,
                    None => &Cell::Dead,
                }
            } else { &Cell::Dead },
            None => &Cell::Dead,
        };
        let bottom = match self.cells.get(row_index + 1) {
            Some(row) => &row[column_index],
            None => &Cell::Dead,
        };
        let bottom_right = match self.cells.get(row_index + 1) {
            Some(row) => match row.get(column_index + 1) {
                Some(cell) => cell,
                None => &Cell::Dead,
            }
            None => &Cell::Dead,
        };
    
        let mut amount_alive = 0;
        if let Cell::Alive = top_left {
            amount_alive += 1;
        }
        if let Cell::Alive = top {
            amount_alive += 1;
        }
        if let Cell::Alive = top_right {
            amount_alive += 1;
        }
        if let Cell::Alive = left {
            amount_alive += 1;
        }
        if let Cell::Alive = right {
            amount_alive += 1;    
        }
        if let Cell::Alive = bottom_left {
            amount_alive += 1;
        }
        if let Cell::Alive = bottom {
            amount_alive += 1;
        }
        if let Cell::Alive = bottom_right {
            amount_alive += 1;
        }
        amount_alive
    }
}

// traits
impl<const WIDTH: usize, const HEIGHT: usize> std::fmt::Display for Board<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            write!(f, "[")?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> std::default::Default for Board<WIDTH, HEIGHT> {
    fn default() -> Self {
        Board { 
            cells: [[Cell::default(); WIDTH]; HEIGHT], 
        }
    }
}
