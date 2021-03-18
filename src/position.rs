use bevy::math::Vec3;

pub const SQUARE_WIDTH: i32 = 50;

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Shape {
    pub squares: Vec<Position>
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

// Pour définir la place que prend une forme (Shape) on définit
// l'ensemble de ses carrés qui font chacun 50 x 50 px
impl Position {
    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, 0.)
    }
}

impl Shape {
    pub fn change_origin(&mut self, new_x: i32, new_y: i32) {

        let mut i = 0;
        for mut square in &mut self.squares {
            square.x = new_x + (i * SQUARE_WIDTH);
            square.y = new_y + (i * SQUARE_WIDTH);
            i = i + 1;
        }
    }
}

pub struct ShapeBuilder {
    pub squares: Vec<Position>
}

impl ShapeBuilder {
    /// * * *
    /// * * *
    /// * * *
    pub fn new_board(start_x: i32, start_y: i32, nb_cols: i32, nb_rows: i32) -> Shape {
        let mut board = Self { squares: vec![] };
        for i in 0..nb_rows {
            board.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), nb_cols));
        };
        board.build()
    }

    /// *
    /// *
    /// *
    pub fn new_rectangle_piece(start_x: i32, start_y: i32) -> Shape {
        let mut rectangle = Self { squares: vec![] };
        for i in 0..3 {
            rectangle.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        rectangle.build()
    }

    /// *
    /// *
    /// * *
    pub fn new_l_piece(start_x: i32, start_y: i32) -> Shape {
        let mut piece = Self { squares: vec![] };
        for i in 0..3 {
            piece.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        piece.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 1));
        piece.build()
    }

    /// * *
    ///   * *
    pub fn new_z_piece(start_x: i32, start_y: i32) -> Shape {
        let mut piece = Self { squares: vec![] };

        piece.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 2));
        piece.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 2));

        piece.build()
    }

    /// *
    /// * *
    pub fn new_corner_piece(start_x: i32, start_y: i32) -> Shape {
        let mut piece = Self { squares: vec![] };

        piece.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y, 2));
        piece.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 1));

        piece.build()
    }

    /// *
    pub fn new_dot_square_piece(start_x: i32, start_y: i32) -> Shape {
        let mut piece = Self { squares: vec![] };

        piece.squares.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y, 1));
        piece.build()
    }

    fn build(self) -> Shape {
        Shape {
            squares: self.squares
        }
    }

    fn new_horizontal_rectangle(start_x: i32, start_y: i32, length: i32) -> Vec<Position> {
        let mut squares = vec![];
        for i in 0..length {
            squares.push(Position {
                x: start_x + (i * SQUARE_WIDTH),
                y: start_y,
            });
        }
        squares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_board() {
        // *
        // *
        let shape = ShapeBuilder::new_board(0, 0, 1, 2);
        assert_eq!(shape.squares, vec![Position { x: 0, y: 0 }, Position { x: 0, y: SQUARE_WIDTH }]);
    }

    #[test]
    fn test_build_rectangle_piece() {
        // *
        // *
        // *
        let shape = ShapeBuilder::new_rectangle_piece(0, 0);
        assert_eq!(shape.squares, vec![
            Position { x: 0, y: 0 },
            Position { x: 0, y: SQUARE_WIDTH },
            Position { x: 0, y: 2 * SQUARE_WIDTH }
        ]);
    }

    #[test]
    fn test_build_l_piece() {
        // *
        // *
        // * *
        let shape = ShapeBuilder::new_l_piece(0, 0);
        assert_eq!(shape.squares, vec![
            Position { x: 0, y: 0 },
            Position { x: 0, y: SQUARE_WIDTH },
            Position { x: 0, y: 2 * SQUARE_WIDTH },
            Position { x: SQUARE_WIDTH, y: 0 }
        ]);
    }

    #[test]
    fn test_build_z_piece() {
        // * *
        //   * *
        let shape = ShapeBuilder::new_z_piece(0, 0);
        assert_eq!(shape.squares, vec![
            Position { x: 0, y: SQUARE_WIDTH },
            Position { x: SQUARE_WIDTH, y: SQUARE_WIDTH },
            Position { x: SQUARE_WIDTH, y: 0 },
            Position { x: 2 * SQUARE_WIDTH, y: 0 }
        ]);
    }

    #[test]
    fn test_build_corner_piece() {
        // *
        // * *
        let shape = ShapeBuilder::new_corner_piece(0, 0);
        assert_eq!(shape.squares, vec![
            Position { x: 0, y: 0 },
            Position { x: SQUARE_WIDTH, y: 0 },
            Position { x: 0, y: SQUARE_WIDTH }
        ]);
    }

    #[test]
    fn test_build_dot_square_piece() {
        // *
        let shape = ShapeBuilder::new_dot_square_piece(0, 0);
        assert_eq!(shape.squares, vec![Position { x: 0, y: 0 }]);
    }

    #[test]
    fn test_change_shape_origin() {
        // * * x
        // x * *
        let mut shape = ShapeBuilder::new_z_piece(0, 0);
        assert_eq!(shape.squares, vec![
            Position { x: 0, y: SQUARE_WIDTH },
            Position { x: SQUARE_WIDTH, y: SQUARE_WIDTH },
            Position { x: SQUARE_WIDTH, y: 0 },
            Position { x: 2 * SQUARE_WIDTH, y: 0 }
        ]);

        // x * *
        // x x * *
        let new_x: i32 = 200;
        let new_y: i32 = 230;
        shape.change_origin(200, 230);
        assert_eq!(shape.squares, vec![
            Position { x: new_x, y: new_y + SQUARE_WIDTH },
            Position { x: new_x + SQUARE_WIDTH, y: new_y + SQUARE_WIDTH },
            Position { x: new_x + SQUARE_WIDTH, y: new_y },
            Position { x: new_x + (2 * SQUARE_WIDTH), y: new_y }
        ]);
    }
}
