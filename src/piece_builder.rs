use bevy::prelude::*;

use crate::{board::Board, piece::Position};

pub const SQUARE_WIDTH: i32 = 50;

pub struct PieceBuilder {
    pub positions: Vec<Vec3>,
}

impl PieceBuilder {
    /// * * *
    /// * * *
    /// * * *
    pub fn new_board(
        commands: &mut Commands,
        material: Handle<ColorMaterial>,
        start_x: i32,
        start_y: i32,
        nb_cols: i32,
        nb_rows: i32,
    ) {
        let mut builder = Self { positions: vec![] };
        let min_x: f32 = start_x as f32;
        let min_y: f32 = start_y as f32;
        let mut max_x: f32 = 0_f32;
        let mut max_y: f32 = 0_f32;
        for i in 0..nb_rows {
            let piece_builder = &mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + (i * SQUARE_WIDTH),
                nb_cols,
                0.,
            );
            for vec in piece_builder.iter() {
                if max_x < vec.x {
                    max_x = vec.x;
                }
                if max_y < vec.y {
                    max_y = vec.y;
                }
            }
            builder.positions.append(piece_builder);
        }
        let entities = builder.build_entities(commands, material);
        let board = Board {
            entities,
            min_x,
            max_x,
            min_y,
            max_y,
        };
        commands.spawn().insert(board);
    }

    // /// *
    // /// *
    // /// * *
    // pub fn new_l_piece(
    //     commands: &mut Commands,
    //     material: Handle<ColorMaterial>,
    //     start_x: i32,
    //     start_y: i32,
    // ) {
    //     let mut builder = Self { positions: vec![] };
    //     for i in 0..3 {
    //         builder
    //             .positions
    //             .append(&mut PieceBuilder::new_horizontal_rectangle(
    //                 start_x,
    //                 start_y + (i * SQUARE_WIDTH),
    //                 1,
    //                 1.,
    //             ));
    //     }
    //     builder
    //         .positions
    //         .append(&mut PieceBuilder::new_horizontal_rectangle(
    //             start_x + SQUARE_WIDTH,
    //             start_y,
    //             1,
    //             1.,
    //         ));
    //     builder.build_piece(commands, material);
    // }

    /// * *
    ///   * *
    // pub fn new_z_piece(
    //     commands: &mut Commands,
    //     material: Handle<ColorMaterial>,
    //     start_x: i32,
    //     start_y: i32,
    // ) {
    //     let mut builder = Self { positions: vec![] };

    //     builder
    //         .positions
    //         .append(&mut PieceBuilder::new_horizontal_rectangle(
    //             start_x,
    //             start_y + SQUARE_WIDTH,
    //             2,
    //             1.0,
    //         ));
    //     builder
    //         .positions
    //         .append(&mut PieceBuilder::new_horizontal_rectangle(
    //             start_x + SQUARE_WIDTH,
    //             start_y,
    //             2,
    //             1.0,
    //         ));

    //     builder.build_piece(commands, material);
    // }

    // /// *
    // /// * *
    // pub fn new_corner_piece(
    //     commands: &mut Commands,
    //     material: Handle<ColorMaterial>,
    //     start_x: i32,
    //     start_y: i32,
    // ) {
    //     let mut builder = Self { positions: vec![] };

    //     builder
    //         .positions
    //         .append(&mut PieceBuilder::new_horizontal_rectangle(
    //             start_x, start_y, 2, 1.,
    //         ));
    //     builder
    //         .positions
    //         .append(&mut PieceBuilder::new_horizontal_rectangle(
    //             start_x,
    //             start_y + SQUARE_WIDTH,
    //             1,
    //             1.0,
    //         ));

    //     builder.build_piece(commands, material);
    // }

    // /// *
    // pub fn new_dot_square_piece(
    //     commands: &mut Commands,
    //     material: Handle<ColorMaterial>,
    //     start_x: i32,
    //     start_y: i32,
    // ) {
    //     let mut builder = Self { positions: vec![] };

    //     builder
    //         .positions
    //         .append(&mut PieceBuilder::new_horizontal_rectangle(
    //             start_x, start_y, 1, 1.0,
    //         ));
    //     builder.build_piece(commands, material);
    // }

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

    pub fn new_horizontal_rectangle(
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
        let mut commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // When
        // *
        // *
        PieceBuilder::new_board(&mut commands, materials, 0, 0, 1, 2);
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
    fn test_min_max_position_build_board() {
        // Given
        let mut world = World::default();
        let mut command_queue = CommandQueue::default();
        let mut commands = Commands::new(&mut command_queue, &world);
        let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        let nb_col = 3;
        let nb_row = 4;
        let start_x = 100;
        let start_y = 50;

        // When
        PieceBuilder::new_board(&mut commands, materials, start_x, start_y, nb_col, nb_row);
        command_queue.apply(&mut world);

        // Then
        let board = world.query::<&Board>().iter(&world).next().unwrap();

        assert_eq!(board.min_x, start_x as f32);
        assert_eq!(board.max_x, (start_x + SQUARE_WIDTH * (nb_col - 1)) as f32);
        assert_eq!(board.min_y, start_y as f32);
        assert_eq!(board.max_y, (start_y + SQUARE_WIDTH * (nb_row - 1)) as f32);
    }

    // #[test]
    // fn test_build_l_piece() {
    //     // Given
    //     let mut world = World::default();
    //     let mut command_queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut command_queue, &world);
    //     let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

    //     // When
    //     // *
    //     // *
    //     // * *
    //     PieceBuilder::new_l_piece(&mut commands, materials, 0, 0);
    //     command_queue.apply(&mut world);

    //     // Then
    //     let results = world
    //         .query_filtered::<&Transform, With<Position>>()
    //         .iter(&world)
    //         .map(|t| t.translation)
    //         .collect::<Vec<_>>();

    //     assert_eq!(
    //         results,
    //         vec![
    //             Vec3::new(0., 0., 1.),
    //             Vec3::new(0., SQUARE_WIDTH as f32, 1.),
    //             Vec3::new(0., 2. * (SQUARE_WIDTH as f32), 1.),
    //             Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
    //         ]
    //     );
    // }

    // #[test]
    // fn test_build_z_piece() {
    //     // Given
    //     let mut world = World::default();
    //     let mut command_queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut command_queue, &world);
    //     let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

    //     // When
    //     // * *
    //     //   * *
    //     PieceBuilder::new_z_piece(&mut commands, materials, 0, 0);
    //     command_queue.apply(&mut world);

    //     // Then
    //     let results = world
    //         .query_filtered::<&Transform, With<Position>>()
    //         .iter(&world)
    //         .map(|t| t.translation)
    //         .collect::<Vec<_>>();

    //     assert_eq!(
    //         results,
    //         vec![
    //             Vec3::new(0.0, SQUARE_WIDTH as f32, 1.0),
    //             Vec3::new(SQUARE_WIDTH as f32, SQUARE_WIDTH as f32, 1.),
    //             Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
    //             Vec3::new(2. * SQUARE_WIDTH as f32, 0., 1.)
    //         ]
    //     );
    // }

    // #[test]
    // fn test_build_corner_piece() {
    //     // Given
    //     let mut world = World::default();
    //     let mut command_queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut command_queue, &world);
    //     let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

    //     // When
    //     // *
    //     // * *
    //     PieceBuilder::new_corner_piece(&mut commands, materials, 0, 0);
    //     command_queue.apply(&mut world);

    //     // Then
    //     let results = world
    //         .query_filtered::<&Transform, With<Position>>()
    //         .iter(&world)
    //         .map(|t| t.translation)
    //         .collect::<Vec<_>>();
    //     assert_eq!(
    //         results,
    //         vec![
    //             Vec3::new(0., 0., 1.),
    //             Vec3::new(SQUARE_WIDTH as f32, 0., 1.),
    //             Vec3::new(0., SQUARE_WIDTH as f32, 1.),
    //         ]
    //     );
    // }

    // #[test]
    // fn test_build_dot_square_piece() {
    //     // Given
    //     let mut world = World::default();
    //     let mut command_queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut command_queue, &world);
    //     let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

    //     // When
    //     // *
    //     PieceBuilder::new_dot_square_piece(&mut commands, materials, 0, 0);
    //     command_queue.apply(&mut world);

    //     // Then
    //     let results = world
    //         .query_filtered::<&Transform, With<Position>>()
    //         .iter(&world)
    //         .map(|t| t.translation)
    //         .collect::<Vec<_>>();
    //     assert_eq!(results, vec![Vec3::new(0., 0., 1.),]);
    // }
}
