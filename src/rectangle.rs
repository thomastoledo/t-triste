use bevy::prelude::*;

use crate::{
    cursor::Cursor,
    piece::Piece,
    piece_builder::{PieceBuilder, SQUARE_WIDTH},
};

pub struct Rectangle {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
}

impl Rectangle {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        let mut positions = vec![];
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

    fn is_horizontal(&self) -> bool {
        let first_y = self.positions.first().unwrap().y;
        self.positions.iter().all(|pos| first_y == pos.y)
    }
    fn is_vertical(&self) -> bool {
        let first_x = self.positions.first().unwrap().x;
        self.positions.iter().all(|pos| first_x == pos.x)
    }
}

impl Piece for Rectangle {
    fn rotate(&mut self) {
        let position_length = self.positions.len();
        let middle_index = position_length / 2;
        let central_piece_position = self.positions[middle_index];

        let compute_delta = |idx| (idx as f32 - middle_index as f32) * SQUARE_WIDTH as f32;

        if self.is_vertical() {
            for (idefix, pos) in self.positions.iter_mut().enumerate() {
                pos.x = central_piece_position.x - compute_delta(idefix);
                pos.y = central_piece_position.y;
            }
        } else if self.is_horizontal() {
            for (idefix, pos) in self.positions.iter_mut().enumerate() {
                pos.y = central_piece_position.y - compute_delta(idefix);
                pos.x = central_piece_position.x;
            }
        }
    }

    fn move_it(&mut self, cursor: &Res<Cursor>) {
        let first_pos = self.positions.first_mut().unwrap();

        let delta_x = -first_pos.x + cursor.current_pos.x;
        let delta_y = -first_pos.y + cursor.current_pos.y;

        first_pos.x = cursor.current_pos.x;
        first_pos.y = cursor.current_pos.y;

        for pos in self.positions.iter_mut().skip(1) {
            pos.x = pos.x + delta_x;
            pos.y = pos.y + delta_y;
        }
    }

    fn positions(&self) -> Vec<Vec3> {
        self.positions.clone()
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn set_moving(&mut self, moving: bool) {
        self.moving = moving;
    }

    fn is_moving(&self) -> bool {
        self.moving
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
