use super::board::Difference;

#[derive(Clone, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Debug)]
pub enum KindOfPiece {
    T4,
    L4,
    Square4,
    Stick4,
    Staires4,
}

pub struct Piece {
    kind: KindOfPiece,
}

impl Piece {
    fn new(kind: KindOfPiece) -> Piece {
        Piece { kind }
    }

    /*
        pub fn get_kind(&self) -> KindOfPiece {
            self.kind.clone()
        }
    */

    pub fn shape(&self, direction: &Direction, reverse: bool) -> Vec<Difference> {
        let mut result: Vec<Difference> = Vec::new();

        for item in self.create_baseshape() {
            let diff = match direction {
                Direction::Top => Difference::new(item.x, item.y),
                Direction::Right => Difference::new(-(item.y), item.x),
                Direction::Bottom => Difference::new(-(item.x), -(item.y)),
                Direction::Left => Difference::new(item.y, -(item.x)),
            };
            result.push(diff);
        }

        if reverse {
            let mut reversed: Vec<Difference> = Vec::new();
            for item in &result {
                let diff = Difference::new(item.y, item.x);
                reversed.push(diff);
            }
            reversed
        } else {
            result
        }
    }

    pub fn get_shape(&self, index: usize) -> Option<Vec<Difference>> {
        let mut result: Vec<Difference> = Vec::new();
        let shapes = self.get_all_shapes();
        let temp = shapes.get(index);
        if temp == None {
            return None;
        } else {
            for i in temp.unwrap() {
                result.push(Difference::new(i.x, i.y));
            }
        }

        Some(result)
    }

    fn create_baseshape(&self) -> Vec<Difference> {
        match self.kind {
            KindOfPiece::T4 => create_baseshape_t4(),
            KindOfPiece::L4 => create_baseshape_l4(),
            KindOfPiece::Square4 => create_baseshape_square4(),
            KindOfPiece::Staires4 => create_baseshape_staires4(),
            KindOfPiece::Stick4 => create_baseshape_stick4(),
        }
    }

    pub fn piece_t4() -> Piece {
        Piece::new(KindOfPiece::T4)
    }

    pub fn piece_l4() -> Piece {
        Piece::new(KindOfPiece::L4)
    }

    pub fn piece_square4() -> Piece {
        Piece::new(KindOfPiece::Square4)
    }

    pub fn piece_stick4() -> Piece {
        Piece::new(KindOfPiece::Stick4)
    }

    pub fn piece_staires4() -> Piece {
        Piece::new(KindOfPiece::Staires4)
    }

    pub fn get_all_shapes(&self) -> Vec<Vec<Difference>> {
        match self.kind {
            KindOfPiece::T4 => create_allshapes_t4(),
            KindOfPiece::L4 => create_allshapes_l4(),
            KindOfPiece::Square4 => create_allshapes_square4(),
            KindOfPiece::Staires4 => create_allshapes_staires4(),
            KindOfPiece::Stick4 => create_allshapes_stick4(),
        }
    }
}

fn create_baseshape_t4() -> Vec<Difference> {
    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 1, y: 1 });
    shape.push(Difference { x: 2, y: 0 });
    shape
}

fn create_baseshape_l4() -> Vec<Difference> {
    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 2, y: 0 });
    shape.push(Difference { x: 2, y: 1 });
    shape
}

fn create_baseshape_square4() -> Vec<Difference> {
    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 0, y: 1 });
    shape.push(Difference { x: 1, y: 1 });
    shape
}

fn create_baseshape_stick4() -> Vec<Difference> {
    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 2, y: 0 });
    shape.push(Difference { x: 3, y: 0 });
    shape
}

fn create_baseshape_staires4() -> Vec<Difference> {
    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 1, y: 1 });
    shape.push(Difference { x: 2, y: 1 });
    shape
}

fn create_allshapes_t4() -> Vec<Vec<Difference>> {
    let mut shapes: Vec<Vec<Difference>> = Vec::new();

    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 1, y: 1 });
    shape.push(Difference { x: 2, y: 0 });
    shapes.push(shape);

    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 0, y: 1 });
    shape.push(Difference { x: 1, y: 1 });
    shape.push(Difference { x: 0, y: 2 });
    shapes.push(shape);

    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 0, y: 1 });
    shape.push(Difference { x: 1, y: -1 });
    shape.push(Difference { x: 0, y: 2 });
    shapes.push(shape);

    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 1, y: -1 });
    shape.push(Difference { x: 2, y: 0 });
    shapes.push(shape);
    shapes
}

fn create_allshapes_l4() -> Vec<Vec<Difference>> {
    let mut shapes: Vec<Vec<Difference>> = Vec::new();

    let l4 = Piece::piece_l4();
    let direction: [Direction; 4] = [
        Direction::Top,
        Direction::Bottom,
        Direction::Left,
        Direction::Right,
    ];
    let reverse: [bool; 2] = [true, false];

    for d in direction.iter() {
        for r in reverse.iter() {
            shapes.push(l4.shape(d, *r));
        }
    }
    shapes
}

fn create_allshapes_square4() -> Vec<Vec<Difference>> {
    let mut shapes: Vec<Vec<Difference>> = Vec::new();

    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 0, y: 1 });
    shape.push(Difference { x: 1, y: 1 });
    shapes.push(shape);
    shapes
}

fn create_allshapes_staires4() -> Vec<Vec<Difference>> {
    let mut shapes: Vec<Vec<Difference>> = Vec::new();

    let l4 = Piece::piece_l4();
    let direction: [Direction; 4] = [
        Direction::Top,
        Direction::Bottom,
        Direction::Left,
        Direction::Right,
    ];

    for d in direction.iter() {
        shapes.push(l4.shape(d, true));
    }
    shapes
}

fn create_allshapes_stick4() -> Vec<Vec<Difference>> {
    let mut shapes: Vec<Vec<Difference>> = Vec::new();

    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 1, y: 0 });
    shape.push(Difference { x: 2, y: 0 });
    shape.push(Difference { x: 3, y: 0 });
    shapes.push(shape);

    let mut shape: Vec<Difference> = Vec::new();
    shape.push(Difference { x: 0, y: 0 });
    shape.push(Difference { x: 0, y: 1 });
    shape.push(Difference { x: 0, y: 2 });
    shape.push(Difference { x: 0, y: 3 });
    shapes.push(shape);
    shapes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_piece4t_shape_top() {
        let p4t = Piece::piece_t4();
        let direction = Direction::Top;
        let reverse = false;

        let actual = p4t.shape(&direction, reverse);
        assert_eq!(Difference { x: 0, y: 0 }, actual[0]);
        assert_eq!(Difference { x: 1, y: 0 }, actual[1]);
        assert_eq!(Difference { x: 1, y: 1 }, actual[2]);
        assert_eq!(Difference { x: 2, y: 0 }, actual[3]);
    }

    #[test]
    fn test_get_piece4t_shape_top_reverse() {
        let p4t = Piece::piece_t4();
        let direction = Direction::Top;
        let reverse = true;
        let actual = p4t.shape(&direction, reverse);
        assert_eq!(Difference { x: 0, y: 0 }, actual[0]);
        assert_eq!(Difference { x: 0, y: 1 }, actual[1]);
        assert_eq!(Difference { x: 1, y: 1 }, actual[2]);
        assert_eq!(Difference { x: 0, y: 2 }, actual[3]);
    }

    #[test]
    fn test_get_piece4t_shape_right() {
        let p4t = Piece::piece_t4();
        let direction = Direction::Right;
        let reverse = false;
        let actual = p4t.shape(&direction, reverse);
        assert_eq!(Difference { x: 0, y: 0 }, actual[0]);
        assert_eq!(Difference { x: 0, y: 1 }, actual[1]);
        assert_eq!(Difference { x: -1, y: 1 }, actual[2]);
        assert_eq!(Difference { x: 0, y: 2 }, actual[3]);
    }

    #[test]
    fn test_get_piece4t_shape_bottom() {
        let p4t = Piece::piece_t4();
        let direction = Direction::Bottom;
        let reverse = false;
        let actual = p4t.shape(&direction, reverse);
        assert_eq!(Difference { x: 0, y: 0 }, actual[0]);
        assert_eq!(Difference { x: -1, y: 0 }, actual[1]);
        assert_eq!(Difference { x: -1, y: -1 }, actual[2]);
        assert_eq!(Difference { x: -2, y: 0 }, actual[3]);
    }

    #[test]
    fn test_get_piece4t_shape_left() {
        let p4t = Piece::piece_t4();
        let direction = Direction::Left;
        let reverse = false;
        let actual = p4t.shape(&direction, reverse);
        assert_eq!(Difference { x: 0, y: 0 }, actual[0]);
        assert_eq!(Difference { x: 0, y: -1 }, actual[1]);
        assert_eq!(Difference { x: 1, y: -1 }, actual[2]);
        assert_eq!(Difference { x: 0, y: -2 }, actual[3]);
    }

    #[test]
    fn test_get_piece4l_shape_right() {
        let p4l = Piece::piece_l4();
        let direction = Direction::Right;
        let reverse = false;
        let actual = p4l.shape(&direction, reverse);
        assert_eq!(Difference { x: 0, y: 0 }, actual[0]);
        assert_eq!(Difference { x: 0, y: 1 }, actual[1]);
        assert_eq!(Difference { x: 0, y: 2 }, actual[2]);
        assert_eq!(Difference { x: -1, y: 2 }, actual[3]);
    }

    #[test]
    fn test_get_piece4l_shape_bottom_reverse() {
        let p4l = Piece::piece_l4();
        let direction = Direction::Bottom;
        let reverse = true;
        let actual = p4l.shape(&direction, reverse);
        assert_eq!(Difference { x: 0, y: 0 }, actual[0]);
        assert_eq!(Difference { x: 0, y: -1 }, actual[1]);
        assert_eq!(Difference { x: 0, y: -2 }, actual[2]);
        assert_eq!(Difference { x: -1, y: -2 }, actual[3]);
    }
}
