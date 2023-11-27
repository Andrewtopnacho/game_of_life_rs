#[derive(Clone, Copy, Default, PartialEq)]
pub enum Cell {
    Alive,
    
    #[default]
    Dead,
}

impl Cell {
    pub fn make_dead(&mut self) {
       *self = Cell::Dead;
    }

    pub fn make_alive(&mut self) {
        *self = Cell::Alive;
    }

    pub fn change_state(&mut self) {
        match self {
            Cell::Dead => self.make_alive(),
            Cell::Alive => self.make_dead(),
        };
    }
    
    pub fn update(&mut self, neighbor_count: usize) {
        match self {
            Cell::Alive => {
                match neighbor_count {
                    0 | 1 => self.make_dead(),
                    2 | 3 => (),
                    _ => self.make_dead()
                }   
            },
            Cell::Dead => {
                match neighbor_count {
                    3 => self.make_alive(),
                    _ => (),
                }
            },
        };
    }
}

// traits
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Alive => "O",
            Cell::Dead => "X",
        })
    }
}
