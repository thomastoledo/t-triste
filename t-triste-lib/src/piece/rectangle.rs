use bevy::prelude::*;
use t_triste_macro::PieceBehavior;

use crate::piece::piece_builder::{PieceBuilder, SQUARE_WIDTH};


#[derive(PieceBehavior)]
pub struct Rectangle {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
}

impl Rectangle {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        let mut positions = vec![];
        // TODO: Use horizontal rectangle from pieceBuilder ?
        for i in 0..3 {
            positions.append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + (i * SQUARE_WIDTH),
                1,
                1.,
            ));
        }
        Rectangle {
            positions,
            color: Color::rgb(0.68, 0.1, 1.03),
            moving: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::vec3;

    use super::*;

    #[test]
    fn test_build_rectangle_piece() {
        // Given
        assert_eq!(true, false, "LEHL");
        // let mut world = World::default();
        // let mut command_queue = CommandQueue::default();
        // let mut commands = Commands::new(&mut command_queue, &world);
        // let materials: Handle<ColorMaterial> = Handle::weak(HandleId::random::<ColorMaterial>());

        // // When
        // // *
        // // *
        // // *
        // PieceBuilder::new_rectangle_piece(&mut commands, materials, 0, 0);
        // command_queue.apply(&mut world);

        // // Then
        // let results = world
        //     .query_filtered::<&Transform, With<Position>>()
        //     .iter(&world)
        //     .map(|t| t.translation)
        //     .collect::<Vec<_>>();
        // assert_eq!(
        //     results,
        //     vec![
        //         Vec3::new(0., 0., 1.),
        //         Vec3::new(0., SQUARE_WIDTH as f32, 1.),
        //         Vec3::new(0., 2. * SQUARE_WIDTH as f32, 1.),
        //     ]
        // );
    }

    #[test]
    fn test_rotate_90() {
        // Given
        let mut rectangle = Rectangle::new(200, 50);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![
                vec3(250., 100., 1.),
                vec3(200., 100., 1.),
                vec3(150., 100., 1.)
            ]
        );
    }

    #[test]
    fn test_rotate_180() {
        // Given
        let mut rectangle = Rectangle::new(200, 50);
        rectangle.positions = vec![
            vec3(250., 100., 0.),
            vec3(200., 100., 0.),
            vec3(150., 100., 0.),
        ];

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![
                vec3(200., 150., 0.),
                vec3(200., 100., 0.),
                vec3(200., 50., 0.)
            ]
        );
    }

    #[test]
    fn test_rotate_270() {
        // Given
        let mut rectangle = Rectangle::new(200, 50);
        rectangle.positions = vec![
            vec3(200., 150., 0.),
            vec3(200., 100., 0.),
            vec3(200., 50., 0.),
        ];

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![
                vec3(250., 100., 0.),
                vec3(200., 100., 0.),
                vec3(150., 100., 0.)
            ]
        );
    }

    #[test]
    fn test_rotate_360() {
        // Given
        let mut rectangle = Rectangle::new(200, 50);
        rectangle.positions = vec![
            vec3(150., 100., 0.),
            vec3(200., 100., 0.),
            vec3(250., 100., 0.),
        ];

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![
                vec3(200., 150., 0.),
                vec3(200., 100., 0.),
                vec3(200., 50., 0.)
            ]
        );
    }
}
