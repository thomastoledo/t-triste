use bevy::prelude::*;

use crate::board::Board;
use crate::piece::Piece;
use crate::position::Position;

pub const SQUARE_WIDTH: i32 = 50;

pub struct PieceBuilder {
    pub positions: Vec<Vec3>,
}

impl PieceBuilder {
    /// * * *
    /// * * *
    /// * * *
    pub fn new_board(
        commands: Commands,
        material: Handle<ColorMaterial>,
        start_x: i32,
        start_y: i32,
        nb_cols: i32,
        nb_rows: i32,
    ) {
        let mut builder = Self { positions: vec![] };
        for i in 0..nb_rows {
            builder
                .positions
                .append(&mut PieceBuilder::new_horizontal_rectangle(
                    start_x,
                    start_y + (i * SQUARE_WIDTH),
                    nb_cols,
                    0.,
                ));
        }
        builder.build_board(commands, material);
    }

    /// *
    /// *
    /// *
    pub fn new_rectangle_piece(
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        start_x: i32,
        start_y: i32,
    ) {
        let mut builder = Self { positions: vec![] };
        for i in 0..3 {
            builder
                .positions
                .append(&mut PieceBuilder::new_horizontal_rectangle(
                    start_x,
                    start_y + (i * SQUARE_WIDTH),
                    1,
                    1.,
                ));
        }
        builder.build_piece(commands, material);
    }

    /// *
    /// *
    /// * *
    pub fn new_l_piece(
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        start_x: i32,
        start_y: i32,
    ) {
        let mut builder = Self { positions: vec![] };
        for i in 0..3 {
            builder
                .positions
                .append(&mut PieceBuilder::new_horizontal_rectangle(
                    start_x,
                    start_y + (i * SQUARE_WIDTH),
                    1,
                    1.,
                ));
        }
        builder
            .positions
            .append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x + SQUARE_WIDTH,
                start_y,
                1,
                1.,
            ));
        builder.build_piece(commands, material);
    }

    /// * *
    ///   * *
    pub fn new_z_piece(
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        start_x: i32,
        start_y: i32,
    ) {
        let mut builder = Self { positions: vec![] };

        builder
            .positions
            .append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + SQUARE_WIDTH,
                2,
                1.0,
            ));
        builder
            .positions
            .append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x + SQUARE_WIDTH,
                start_y,
                2,
                1.0,
            ));

        builder.build_piece(commands, material);
    }

    /// *
    /// * *
    pub fn new_corner_piece(
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        start_x: i32,
        start_y: i32,
    ) {
        let mut builder = Self { positions: vec![] };

        builder
            .positions
            .append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x, start_y, 2, 1.,
            ));
        builder
            .positions
            .append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + SQUARE_WIDTH,
                1,
                1.0,
            ));

        builder.build_piece(commands, material);
    }

    /// *
    pub fn new_dot_square_piece(
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        start_x: i32,
        start_y: i32,
    ) {
        let mut builder = Self { positions: vec![] };

        builder
            .positions
            .append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x, start_y, 1, 1.0,
            ));
        builder.build_piece(commands, material);
    }

    fn build_piece(mut self, commands: &mut Commands, material: Handle<ColorMaterial>) {
        let entities = self.build_entities(commands, material);
        let piece = Piece {
            entities,
            rotation: 0_f32,
        };
        commands.spawn().insert(piece);
    }

    fn build_board(mut self, mut commands: Commands, material: Handle<ColorMaterial>) {
        let entities = self.build_entities(&mut commands, material);
        let board = Board { entities };
        commands.spawn().insert(board);
    }

    fn build_entities(
        &mut self,
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
    ) -> Vec<Entity> {
        self.positions
            .iter_mut()
            .map(|position| {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: material.clone(),
                        sprite: Sprite::new(Vec2::new(
                            (SQUARE_WIDTH - 1) as f32,
                            (SQUARE_WIDTH - 1) as f32,
                        )),
                        transform: Transform::from_translation(*position),
                        ..Default::default()
                    })
                    .insert(Position)
                    .id()
            })
            .collect()
    }

    fn new_horizontal_rectangle(
        start_x: i32,
        start_y: i32,
        length: i32,
        z_index: f32,
    ) -> Vec<Vec3> {
        let mut squares = vec![];
        for i in 0..length {
            squares.push(Vec3::new(
                (start_x + i * SQUARE_WIDTH) as f32,
                start_y as f32,
                z_index,
            ))
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
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 0.),
                Vec3::new(0., SQUARE_WIDTH as f32, 0.)
            ]
        );
    }

    #[test]
    fn test_build_rectangle_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        // *
        PieceBuilder::new_rectangle_piece(&mut commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(0., SQUARE_WIDTH as f32, 1.),
                Vec3::new(0., 2. * SQUARE_WIDTH as f32, 1.),
            ]
        );
    }

    #[test]
    fn test_build_l_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        // * *
        PieceBuilder::new_l_piece(&mut commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(0., SQUARE_WIDTH as f32, 1.),
                Vec3::new(0., 2. * (SQUARE_WIDTH as f32), 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
            ]
        );
    }

    #[test]
    fn test_build_z_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // * *
        //   * *
        PieceBuilder::new_z_piece(&mut commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Vec3::new(0.0, SQUARE_WIDTH as f32, 1.0),
                Vec3::new(SQUARE_WIDTH as f32, SQUARE_WIDTH as f32, 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(2. * SQUARE_WIDTH as f32, 0., 1.)
            ]
        );
    }

    #[test]
    fn test_build_corner_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // * *
        PieceBuilder::new_corner_piece(&mut commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![
                Vec3::new(0., 0., 1.),
                Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
                Vec3::new(0., SQUARE_WIDTH as f32, 1.),
            ]
        );
    }

    #[test]
    fn test_build_dot_square_piece() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        PieceBuilder::new_dot_square_piece(&mut commands, materials, 0, 0);
        command_queue.apply(&mut world);

        // Then
        let results = world
            .query_filtered::<&Transform, With<Position>>()
            .iter(&world)
            .map(|t| t.translation)
            .collect::<Vec<_>>();
        assert_eq!(results, vec![Vec3::new(0., 0., 1.),]);
    }
}
