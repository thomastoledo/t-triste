use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::reflect::List;

pub const SQUARE_WIDTH: i32 = 50;

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Shape {
    pub entities: Vec<Entity>
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
    // pub fn change_origin(&mut self, new_x: i32, new_y: i32) {
    //     let mut i = 0;
    //     for mut square in &mut self.entities {
    //         square.x = new_x + (i * SQUARE_WIDTH);
    //         square.y = new_y + (i * SQUARE_WIDTH);
    //         i = i + 1;
    //     }
    // }
}

pub struct ShapeBuilder {
    pub positions: Vec<Position>
}

impl ShapeBuilder {
    /// * * *
    /// * * *
    /// * * *
    pub fn new_board(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32, nb_cols: i32, nb_rows: i32) {
        let mut board = Self { positions: vec![] };
        for i in 0..nb_rows {
            board.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), nb_cols));
        };
        board.build(commands, material);
    }

    /// *
    /// *
    /// *
    pub fn new_rectangle_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut rectangle = Self { positions: vec![] };
        for i in 0..3 {
            rectangle.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        rectangle.build(commands, material);
    }

    /// *
    /// *
    /// * *
    pub fn new_l_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };
        for i in 0..3 {
            piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 1));
        piece.build(commands, material);
    }

    /// * *
    ///   * *
    pub fn new_z_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };

        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 2));
        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 2));

        piece.build(commands, material);
    }

    /// *
    /// * *
    pub fn new_corner_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };

        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y, 2));
        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 1));

        piece.build(commands, material);
    }

    /// *
    pub fn new_dot_square_piece(commands: &mut Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut piece = Self { positions: vec![] };

        piece.positions.append(&mut ShapeBuilder::new_horizontal_rectangle(start_x, start_y, 1));
        piece.build(commands, material);
    }

    fn build(&mut self, commands: &mut Commands, materials: Handle<ColorMaterial>) {
        let entities: Vec<Entity> = self.positions.iter_mut().map(|position| {
            commands.spawn(
                SpriteBundle {
                    material: materials.clone(),
                    sprite: Sprite::new(Vec2::new(SQUARE_WIDTH as f32, SQUARE_WIDTH as f32)),
                    transform: Transform::from_translation(position.to_vec()),
                    ..Default::default()
                }
            )
                .with(*position)
                .current_entity()
                .unwrap()
        }).collect();

        let shape = Shape {
            entities
        };
        commands.with(shape);
        // shape
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
    use bevy::asset::HandleId;

    use super::*;

    #[test]
    fn test_build_board() {
        // Given
        // TODO: This is probably the broken part
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut commands = Commands::default();
        let mut materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());
        commands.set_entity_reserver(world.get_entity_reserver());

        // When
        // *
        // *
        ShapeBuilder::new_board(&mut commands, materials, 0, 0, 1, 2);

        // Then
        // TODO: Fix this test
        let toto = world.query::<&Shape>().collect::<Vec<_>>();
        println!("{:?}", toto);
        let results = world.query::<&Position>().collect::<Vec<_>>();
        assert_eq!(results, vec![&Position { x: 0, y: 0 }, &Position { x: 0, y: SQUARE_WIDTH }]);
    }

    // #[test]
    // fn test_build_rectangle_piece() {
    //     // *
    //     // *
    //     // *
    //     let shape = ShapeBuilder::new_rectangle_piece(0, 0);
    //     assert_eq!(shape.squares, vec![
    //         Position { x: 0, y: 0 },
    //         Position { x: 0, y: SQUARE_WIDTH },
    //         Position { x: 0, y: 2 * SQUARE_WIDTH }
    //     ]);
    // }
    //
    // #[test]
    // fn test_build_l_piece() {
    //     // *
    //     // *
    //     // * *
    //     let shape = ShapeBuilder::new_l_piece(0, 0);
    //     assert_eq!(shape.squares, vec![
    //         Position { x: 0, y: 0 },
    //         Position { x: 0, y: SQUARE_WIDTH },
    //         Position { x: 0, y: 2 * SQUARE_WIDTH },
    //         Position { x: SQUARE_WIDTH, y: 0 }
    //     ]);
    // }
    //
    // #[test]
    // fn test_build_z_piece() {
    //     // * *
    //     //   * *
    //     let shape = ShapeBuilder::new_z_piece(0, 0);
    //     assert_eq!(shape.squares, vec![
    //         Position { x: 0, y: SQUARE_WIDTH },
    //         Position { x: SQUARE_WIDTH, y: SQUARE_WIDTH },
    //         Position { x: SQUARE_WIDTH, y: 0 },
    //         Position { x: 2 * SQUARE_WIDTH, y: 0 }
    //     ]);
    // }
    //
    // #[test]
    // fn test_build_corner_piece() {
    //     // *
    //     // * *
    //     let shape = ShapeBuilder::new_corner_piece(0, 0);
    //     assert_eq!(shape.squares, vec![
    //         Position { x: 0, y: 0 },
    //         Position { x: SQUARE_WIDTH, y: 0 },
    //         Position { x: 0, y: SQUARE_WIDTH }
    //     ]);
    // }
    //
    // #[test]
    // fn test_build_dot_square_piece() {
    //     // *
    //     let shape = ShapeBuilder::new_dot_square_piece(0, 0);
    //     assert_eq!(shape.squares, vec![Position { x: 0, y: 0 }]);
    // }
    //
    // #[test]
    // fn test_change_shape_origin() {
    //     // * * x
    //     // x * *
    //     let mut shape = ShapeBuilder::new_z_piece(0, 0);
    //     assert_eq!(shape.squares, vec![
    //         Position { x: 0, y: SQUARE_WIDTH },
    //         Position { x: SQUARE_WIDTH, y: SQUARE_WIDTH },
    //         Position { x: SQUARE_WIDTH, y: 0 },
    //         Position { x: 2 * SQUARE_WIDTH, y: 0 }
    //     ]);
    //
    //     // x * *
    //     // x x * *
    //     let new_x: i32 = 200;
    //     let new_y: i32 = 230;
    //     shape.change_origin(200, 230);
    //     assert_eq!(shape.squares, vec![
    //         Position { x: new_x, y: new_y + SQUARE_WIDTH },
    //         Position { x: new_x + SQUARE_WIDTH, y: new_y + SQUARE_WIDTH },
    //         Position { x: new_x + SQUARE_WIDTH, y: new_y },
    //         Position { x: new_x + (2 * SQUARE_WIDTH), y: new_y }
    //     ]);
    // }
}
