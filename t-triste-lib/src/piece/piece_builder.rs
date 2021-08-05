use bevy::prelude::*;

use crate::piece::SQUARE_WIDTH;

pub struct PieceBuilder {
    pub positions: Vec<Vec3>,
}

impl PieceBuilder {
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

// #[cfg(test)]
// mod tests {
//     use bevy::asset::HandleId;
//     use bevy::ecs::system::CommandQueue;

//     use super::*;

//     #[test]
//     fn test_build_board() {
//         // Given
//         let mut world = World::default();
//         let mut command_queue = CommandQueue::default();
//         let mut commands = Commands::new(&mut command_queue, &world);
//         let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

//         // When
//         // *
//         // *
//         PieceBuilder::new_board(&mut commands, materials, 0, 0, 1, 2);
//         command_queue.apply(&mut world);

//         // Then
//         let results = world
//             .query_filtered::<&Transform, With<Position>>()
//             .iter(&world)
//             .map(|t| t.translation)
//             .collect::<Vec<_>>();

//         assert_eq!(
//             results,
//             vec![
//                 Vec3::new(0., 0., 0.),
//                 Vec3::new(0., SQUARE_WIDTH as f32, 0.)
//             ]
//         );
//     }

//     #[test]
//     fn test_min_max_position_build_board() {
//         // Given
//         let mut world = World::default();
//         let mut command_queue = CommandQueue::default();
//         let mut commands = Commands::new(&mut command_queue, &world);
//         let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

//         let nb_col = 3;
//         let nb_row = 4;
//         let start_x = 100;
//         let start_y = 50;

//         // When
//         PieceBuilder::new_board(&mut commands, materials, start_x, start_y, nb_col, nb_row);
//         command_queue.apply(&mut world);

//         // Then
//         let board = world.query::<&Board>().iter(&world).next().unwrap();

//         assert_eq!(board.min_x, start_x as f32);
//         assert_eq!(board.max_x, (start_x + SQUARE_WIDTH * (nb_col - 1)) as f32);
//         assert_eq!(board.min_y, start_y as f32);
//         assert_eq!(board.max_y, (start_y + SQUARE_WIDTH * (nb_row - 1)) as f32);
//     }

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
// }
