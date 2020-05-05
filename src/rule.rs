use super::board::Board;
//use super::board::Difference;
use super::board::Location;
use super::pieces::Direction;
//use super::pieces::KindOfPiece;
use super::pieces::Piece;

#[derive(Clone, Debug)]
pub struct InputData {
    pub piece: usize,
    pub location: Location,
    pub direction: Direction,
    pub reverse: bool,
}

#[derive(Clone, Debug)]
pub struct InputData2 {
    pub piece: usize,
    pub location: Location,
    pub index: usize,
}

pub struct Rule {
    pub board: Board,
    turn: usize,
    players: Vec<Player>,
    player_number: usize,
}

struct Player {
    pieces: Vec<Piece>,
}

impl Rule {
    pub fn new() -> Rule {
        let player_number = 4;
        let mut board = Board::new(10, 8);

        board.initialize_start_pieces();

        let mut rule = Rule {
            board,
            turn: 1,
            players: Vec::new(),
            player_number,
        };

        for _i in 0..player_number {
            let mut player = Player { pieces: Vec::new() };
            player.pieces.push(Piece::piece_t4());
            player.pieces.push(Piece::piece_l4());
            player.pieces.push(Piece::piece_square4());
            player.pieces.push(Piece::piece_stick4());
            player.pieces.push(Piece::piece_staires4());

            rule.players.push(player);
        }

        rule
    }

    /*
        pub fn put_piece(&mut self, input: InputData) -> bool {
            let piece = &self.players[self.turn - 1].pieces[input.piece];
            if !self.is_able_to_put_piece(&piece, &input.location, &input.direction, input.reverse) {
                return false;
            }

            if self
                .board
                .put_piece(input.location, piece.shape(&input.direction, input.reverse), self.turn)
            {
                self.players[self.turn - 1].pieces.remove(input.piece);
                self.next_turn();
                return true;
            }
            false
        }
    */

    pub fn put_piece2(&mut self, input: InputData2) -> bool {
        let piece = &self.players[self.turn - 1].pieces[input.piece];
        if !self.is_able_to_put_piece2(&piece, &input.location, input.index) {
            return false;
        }

        if self.board.put_piece(
            input.location,
            piece.get_shape(input.index).unwrap(),
            self.turn,
        ) {
            self.players[self.turn - 1].pieces.remove(input.piece);
            self.next_turn();
            return true;
        }
        false
    }

    pub fn get_cell(&self, location: &Location) -> Option<usize> {
        self.board.get_cell(location)
    }

    /*
        fn is_able_to_put_piece(&self, piece: &Piece, location: &Location, direction: &Direction, reverse: bool) -> bool {
            let mut result = false;
            for i in piece.shape(direction, reverse) {
                let temp_loc = Location::new(location.x + i.x, location.y + i.y);
                if self.is_on_the_side_of_piece(&temp_loc) {
                    return false;
                }

                if self.is_on_the_apex_of_piece(&temp_loc) {
                    result = true;
                }

                if self.board.is_out_of_board(&temp_loc) {
                    return false;
                }

                if self.board.is_on_the_piece(&temp_loc) {
                    return false;
                }
            }
            result
        }
    */

    fn is_able_to_put_piece2(&self, piece: &Piece, location: &Location, index: usize) -> bool {
        let mut result = false;
        for i in piece.get_shape(index).unwrap() {
            let temp_loc = Location::new(location.x + i.x, location.y + i.y);
            if self.is_on_the_side_of_piece(&temp_loc) {
                return false;
            }

            if self.is_on_the_apex_of_piece(&temp_loc) {
                result = true;
            }

            if self.board.is_out_of_board(&temp_loc) {
                return false;
            }

            if self.board.is_on_the_piece(&temp_loc) {
                return false;
            }
        }
        result
    }

    fn is_on_the_apex_of_piece(&self, target: &Location) -> bool {
        let check_loc = Location::new(target.x + 1, target.y + 1);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }

        let check_loc = Location::new(target.x + 1, target.y - 1);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }

        let check_loc = Location::new(target.x - 1, target.y + 1);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }

        let check_loc = Location::new(target.x - 1, target.y - 1);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }
        false
    }

    fn is_on_the_side_of_piece(&self, target: &Location) -> bool {
        let check_loc = Location::new(target.x + 1, target.y);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }

        let check_loc = Location::new(target.x - 1, target.y);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }

        let check_loc = Location::new(target.x, target.y + 1);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }

        let check_loc = Location::new(target.x, target.y - 1);
        if self.board.get_cell(&check_loc) == Some(self.turn) {
            return true;
        }
        false
    }

    /*
        pub fn get_enable_to_put_piece(&self) -> Vec<InputData> {
            let mut result: Vec<InputData> = Vec::new();
            for x in 0..10 {
                for y in 0..8 {
                    let number = self.get_number_of_pieces(self.turn);
                    for p in 0..number {
                        for d in 0..4 {
                            for r in 0..2 {
                                let rr = match r {
                                    0 => true,
                                    1 => false,
                                    _ => true,
                                };
                                let loc_copy = Location::new(x, y);
                                let dd = match d {
                                    0 => Direction::Bottom,
                                    1 => Direction::Top,
                                    2 => Direction::Left,
                                    3 => Direction::Right,
                                    _ => Direction::Bottom,
                                };
                                let dd2 = match d {
                                    0 => Direction::Bottom,
                                    1 => Direction::Top,
                                    2 => Direction::Left,
                                    3 => Direction::Right,
                                    _ => Direction::Bottom,
                                };

                                let input = InputData {
                                    piece: p,
                                    location: loc_copy,
                                    direction: dd,
                                    reverse: rr,
                                };

                                let orig_piece = &self.players[self.turn - 1].pieces[p];
    /*
                                let copy_piece = match orig_piece.get_kind() {
                                    KindOfPiece::L4 => Piece::piece_l4(),
                                    KindOfPiece::T4 => Piece::piece_t4(),
                                    KindOfPiece::Square4 => Piece::piece_square4(),
                                    KindOfPiece::Staires4 => Piece::piece_staires4(),
                                    KindOfPiece::Stick4 => Piece::piece_stick4(),
                                };
    */
                                if self.is_able_to_put_piece(&orig_piece, &input.location, &dd2, rr) {
                                    result.push(input);
                                }
                            }
                        }
                    }
                }
            }
            result
        }
    */

    /*
        pub fn get_turn(&self) -> usize {
            return self.turn;
        }
    */

    pub fn get_enable_to_put_piece2(&self) -> Vec<InputData2> {
        let mut result: Vec<InputData2> = Vec::new();
        for x in 0..10 {
            for y in 0..10 {
                let number = self.get_number_of_pieces(self.turn);
                for p in 0..number {
                    let piece = &self.players[self.turn - 1].pieces[p];
                    let len = piece.get_all_shapes().len();
                    for s in 0..len {
                        let loc = Location::new(x, y);
                        let input = InputData2 {
                            piece: p,
                            location: loc,
                            index: s,
                        };

                        if self.is_able_to_put_piece2(&piece, &input.location, input.index) {
                            result.push(input);
                        }
                    }
                }
            }
        }
        result
    }

    /*
    fn is_able_to_put_piece2(&self, shape: &Vec<Difference>, location: &Location) -> bool {
        let mut result = false;
        for i in shape.iter() {
            let temp_loc = Location::new(location.x + i.x, location.y + i.y);
            if self.is_on_the_side_of_piece(&temp_loc) {
                return false;
            }

            if self.is_on_the_apex_of_piece(&temp_loc) {
                result = true;
            }

            if self.board.is_out_of_board(&temp_loc) {
                return false;
            }

            if self.board.is_on_the_piece(&temp_loc) {
                return false;
            }
        }
        result
    }
    */

    pub fn get_number_of_pieces(&self, kind: usize) -> usize {
        self.players[kind - 1].pieces.len()
    }

    /*
        pub fn pass(&mut self) {
            self.next_turn();
        }
    */

    fn next_turn(&mut self) {
        self.turn += 1;
        if self.turn > self.player_number {
            self.turn = 1;
        }
    }
}

#[cfg(test)]

mod test {
    /*
        use super::*;

        #[test]
        fn test_is_piece() {
            let mut rule = Rule::new();
            let piece = 1;
            let location = Location::new(0, 0);
            let direction = Direction::Top;
            let reverse = false;

            let input = InputData {
                piece,
                location,
                direction,
                reverse,
            };

            let result = rule.put_piece(input);
            assert_eq!(true, result);

            let test_loc1 = Location::new(0, 0);
            assert_eq!(Some(1), rule.get_cell(&test_loc1));
        }
    */
}
