use bevy::math::Vec3;

const SQUARE_HEIGHT: i32 = 50;
const SQUARE_WIDTH: i32 = 50;

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Shape {
    pub squares: Vec<Position>
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn to_vec(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, 0.)
    }
}

impl Shape {
    pub fn new() -> Self {
        Self {
            squares: vec![]
        }
    }

    pub fn new_horizontal_rectangle(&mut self, start_x: i32, start_y: i32, length: i32) -> &mut Self {
        for i in 0..length {
            self.squares.push(Position {
                x: start_x + (i * SQUARE_WIDTH),
                y: start_y,
            });
        }
        self
    }

    pub fn new_vertical_rectangle(&mut self, start_x: i32, start_y: i32, length: i32) -> &mut Self {
        for i in 0..length {
            self.squares.push(Position {
                x: start_x,
                y: start_y + (i * SQUARE_HEIGHT),
            });
        }
        self
    }
}

// Pour définir la place que prend une forme (Shape) on définit
// l'ensemble de ces carrés qui font chacun 50 x 50 px
