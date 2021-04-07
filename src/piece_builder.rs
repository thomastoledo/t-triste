use bevy::prelude::*;

use crate::board::Board;
use crate::piece::Piece;
use crate::position::Position;

pub const SQUARE_WIDTH: i32 = 50;

pub struct PieceBuilder {
    pub positions: Vec<Position>
}

impl PieceBuilder {
    /// * * *
    /// * * *
    /// * * *
    pub fn new_board(commands: Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32, nb_cols: i32, nb_rows: i32) {
        let mut builder = Self { positions: vec![] };
        for i in 0..nb_rows {
            builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), nb_cols));
        };
        builder.build_board(commands, material);
    }

    /// *
    /// *
    /// *
    pub fn new_rectangle_piece(commands: Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut builder = Self { positions: vec![] };
        for i in 0..3 {
            builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        builder.build_piece(commands, material);
    }

    /// *
    /// *
    /// * *
    pub fn new_l_piece(commands: Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut builder = Self { positions: vec![] };
        for i in 0..3 {
            builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x, start_y + (i * SQUARE_WIDTH), 1));
        };
        builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 1));
        builder.build_piece(commands, material);
    }

    /// * *
    ///   * *
    pub fn new_z_piece(commands: Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut builder = Self { positions: vec![] };

        builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 2));
        builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x + SQUARE_WIDTH, start_y, 2));

        builder.build_piece(commands, material);
    }

    /// *
    /// * *
    pub fn new_corner_piece(commands: Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut builder = Self { positions: vec![] };

        builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x, start_y, 2));
        builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x, start_y + SQUARE_WIDTH, 1));

        builder.build_piece(commands, material);
    }

    /// *
    pub fn new_dot_square_piece(commands: Commands, material: Handle<ColorMaterial>, start_x: i32, start_y: i32) {
        let mut builder = Self { positions: vec![] };

        builder.positions.append(&mut PieceBuilder::new_horizontal_rectangle(start_x, start_y, 1));
        builder.build_piece(commands, material);
    }

    fn build_piece(mut self, mut commands: Commands, material: Handle<ColorMaterial>) {
        let entities = self.build_entities(&mut commands, material);
        let piece = Piece {
            entities,
            rotation: 0_f32,
            moving: false,
        };
        commands.spawn().insert(piece);
    }

    fn build_board(mut self, mut commands: Commands, material: Handle<ColorMaterial>) {
        let entities = self.build_entities(&mut commands, material);
        let board = Board { entities };
        commands.spawn().insert(board);
    }

    fn build_entities(&mut self, commands: &mut Commands, material: Handle<ColorMaterial>) -> Vec<Entity> {
        self.positions.iter_mut().map(|position| {
            commands.spawn_bundle(
                SpriteBundle {
                    material: material.clone(),
                    sprite: Sprite::new(Vec2::new((SQUARE_WIDTH - 1) as f32, (SQUARE_WIDTH - 1) as f32)),
                    transform: Transform::from_translation(position.to_vec()),
                    ..Default::default()
                }
            )
                .insert(*position)
                .id()
        }).collect()
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
    use bevy::ecs::system::CommandQueue;

    use super::*;

    #[test]
    fn test_build_board() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        PieceBuilder::new_board(commands, materials, 0, 0, 1, 2);
        command_queue.apply(&mut world);

        // Then
        let results = world.query::<&Position>().iter(&world).collect::<Vec<_>>();
        assert_eq!(results, vec![&Position { x: 0, y: 0 }, &Position { x: 0, y: SQUARE_WIDTH }]);
    }

    #[test]
    fn test_build_rectangle_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        // *
        PieceBuilder::new_rectangle_piece(commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query::<&Position>()
            .iter(&world)
            .collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: 0 },
            &Position { x: 0, y: SQUARE_WIDTH },
            &Position { x: 0, y: 2 * SQUARE_WIDTH }
        ]);
    }

    #[test]
    fn test_build_l_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        // * *
        PieceBuilder::new_l_piece(commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query::<&Position>()
            .iter(&world)
            .collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: 0 },
            &Position { x: 0, y: SQUARE_WIDTH },
            &Position { x: 0, y: 2 * SQUARE_WIDTH },
            &Position { x: SQUARE_WIDTH, y: 0 }
        ]);
    }

    #[test]
    fn test_build_z_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // * *
        //   * *
        PieceBuilder::new_z_piece(commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query::<&Position>()
            .iter(&world)
            .collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: SQUARE_WIDTH },
            &Position { x: SQUARE_WIDTH, y: SQUARE_WIDTH },
            &Position { x: SQUARE_WIDTH, y: 0 },
            &Position { x: 2 * SQUARE_WIDTH, y: 0 }
        ]);
    }

    #[test]
    fn test_build_corner_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // * *
        PieceBuilder::new_corner_piece(commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query::<&Position>()
            .iter(&world)
            .collect::<Vec<_>>();
        assert_eq!(results, vec![
            &Position { x: 0, y: 0 },
            &Position { x: SQUARE_WIDTH, y: 0 },
            &Position { x: 0, y: SQUARE_WIDTH }
        ]);
    }

    #[test]
    fn test_build_dot_square_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        PieceBuilder::new_dot_square_piece(commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query::<&Position>()
            .iter(&world)
            .collect::<Vec<_>>();
        assert_eq!(results, vec![&Position { x: 0, y: 0 }]);
    }
}
