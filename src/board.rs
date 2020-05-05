#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Difference {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Location {
        Location { x, y }
    }
}

impl Difference {
    pub fn new(x: i32, y: i32) -> Difference {
        Difference { x, y }
    }
}

struct Cell {
    location: Location,
    kind: usize,
}

pub struct Board {
    cells: Vec<Cell>,
    row_size: i32,
    column_size: i32,
}

impl Board {
    pub fn new(x: i32, y: i32) -> Board {
        Board {
            cells: Vec::new(),
            row_size: x,
            column_size: y,
        }
    }

    pub fn initialize_start_pieces(&mut self) {
        let blue_cell = Cell {
            location: Location::new(-1, -1),
            kind: 1,
        };
        let red_cell = Cell {
            location: Location::new(self.row_size, -1),
            kind: 2,
        };
        let yellow_cell = Cell {
            location: Location::new(-1, self.column_size),
            kind: 3,
        };
        let green_cell = Cell {
            location: Location::new(self.row_size, self.column_size),
            kind: 4,
        };

        self.cells.push(blue_cell);
        self.cells.push(red_cell);
        self.cells.push(yellow_cell);
        self.cells.push(green_cell);
    }

    pub fn put_piece(&mut self, location: Location, shape: Vec<Difference>, kind: usize) -> bool {
        for item in &shape {
            let temp_loc = Location {
                x: location.x + item.x,
                y: location.y + item.y,
            };

            if self.is_out_of_board(&temp_loc) {
                return false;
            }
            if self.is_on_the_piece(&temp_loc) {
                return false;
            }
        }

        for item in &shape {
            let temp_loc = Location {
                x: location.x + item.x,
                y: location.y + item.y,
            };
            let temp_cell = Cell {
                location: temp_loc,
                kind,
            };

            self.cells.push(temp_cell);
        }
        true
    }

    pub fn get_cell(&self, location: &Location) -> Option<usize> {
        /* NG start pices is out of board */
        /*
        if self.is_out_of_board(location) {
            return None;
        }
        */

        for item in &self.cells {
            if item.location == *location {
                return Some(item.kind);
            }
        }
        Some(0)
    }

    pub fn is_out_of_board(&self, location: &Location) -> bool {
        if location.x < 0 {
            return true;
        }
        if location.y < 0 {
            return true;
        }
        if location.x >= self.row_size {
            return true;
        }
        if location.y >= self.column_size {
            return true;
        }
        false
    }

    pub fn is_on_the_piece(&self, target: &Location) -> bool {
        for item in &self.cells {
            if item.location.x == target.x && item.location.y == target.y {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]

mod test {
    use super::super::pieces::Direction;
    use super::super::pieces::Piece;
    use super::*;

    #[test]
    fn test_is_out_of_board() {
        let board = Board {
            cells: Vec::new(),
            row_size: 10,
            column_size: 8,
        };

        let false_location_top_left = Location::new(0, 0);
        let false_location_top_right = Location::new(9, 0);
        let false_location_bottom_left = Location::new(0, 7);
        let false_location_bottom_right = Location::new(9, 7);
        let true_location_top = Location::new(0, -1);
        let true_location_bottom = Location::new(5, 8);
        let true_location_right = Location::new(-1, 2);
        let true_location_left = Location::new(10, 6);

        assert_eq!(false, board.is_out_of_board(&false_location_top_left));
        assert_eq!(false, board.is_out_of_board(&false_location_top_right));
        assert_eq!(false, board.is_out_of_board(&false_location_bottom_left));
        assert_eq!(false, board.is_out_of_board(&false_location_bottom_right));
        assert_eq!(true, board.is_out_of_board(&true_location_top));
        assert_eq!(true, board.is_out_of_board(&true_location_bottom));
        assert_eq!(true, board.is_out_of_board(&true_location_right));
        assert_eq!(true, board.is_out_of_board(&true_location_left));
    }

    #[test]
    fn test_put_piece() {
        let mut board = Board {
            cells: Vec::new(),
            row_size: 10,
            column_size: 8,
        };

        board.initialize_start_pieces();

        let p4t = Piece::piece_t4();
        let direction = Direction::Top;
        let location = Location::new(0, 0);
        board.put_piece(location, p4t.shape(&direction, false), 1);

        let check_location1 = Location::new(0, 0);
        let check_location2 = Location::new(1, 0);
        let check_location3 = Location::new(2, 0);
        let check_location4 = Location::new(1, 1);
        assert_eq!(Some(1), board.get_cell(&check_location1));
        assert_eq!(Some(1), board.get_cell(&check_location2));
        assert_eq!(Some(1), board.get_cell(&check_location3));
        assert_eq!(Some(1), board.get_cell(&check_location4));
    }
}
