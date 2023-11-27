use rand::random;
use crate::cell::Cell;

const COLUMN_COUNT: usize = 10;
const ROW_COUNT: usize = 10;

#[derive(Clone, Copy)]
pub struct Board {
    cells: [[Cell; COLUMN_COUNT]; ROW_COUNT],
}
impl std::fmt::Display for Board {
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
impl std::default::Default for Board {
    fn default() -> Self {
        Board { 
            cells: [[Cell::default(); COLUMN_COUNT]; ROW_COUNT], 
        }
    }
}
impl Board {
    pub fn random() -> Board {
        let mut random_board = Board::default();
        for row in &mut random_board.cells {
            for cell in row {
                if random::<usize>() % 2 == 0 {
                    cell.change_state();
                }
            }
        }
        random_board
    }
}
impl Board {
    pub fn update(&mut self) {
        let buffer = self.clone();
        for row_index in 0..ROW_COUNT {
            for column_index in 0..COLUMN_COUNT {
                let neighbor_count = buffer.get_neighbor_count(row_index, column_index);
                self.cells[row_index][column_index].update(neighbor_count); 
            }
        }
    }

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
