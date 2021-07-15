use bevy::prelude::*;

use crate::cursor::Cursor;

use super::piece_builder::SQUARE_WIDTH;

// Components
pub struct Position;

pub trait Piece {
    fn positions(&self) -> Vec<Vec3>;
    fn color(&self) -> Color;
    fn rotate(&mut self);
    fn move_it(&mut self, cursor: &Res<Cursor>);
    fn set_moving(&mut self, moving: bool);
    fn is_moving(&self) -> bool;

    fn is_even_odd(&self, current_pos: Vec2) -> bool {
        self.positions().iter().any(|piece_pos| {
            piece_pos.x - (SQUARE_WIDTH / 2) as f32 <= current_pos.x
                && current_pos.x <= piece_pos.x + (SQUARE_WIDTH / 2) as f32
                && piece_pos.y - (SQUARE_WIDTH / 2) as f32 <= current_pos.y
                && current_pos.y <= piece_pos.y + (SQUARE_WIDTH / 2) as f32
        })
    }
}


// fn incrust_in_board(
//     mut commands: Commands,
//     mouse_button_input: Res<Input<MouseButton>>,
//     boards: Query<&Board>,
//     pieces: Query<(&Piece, Entity), With<Moving>>,
//     mut positions: Query<&mut Transform, With<Position>>,
// ) {
//     if !mouse_button_input.just_released(MouseButton::Left) {
//         return;
//     }

//     // We know for sure that we only have one board
//     let board = boards.iter().next().unwrap();
//     // TODO: algo to move each transform in the board.
//     let mut piece_transforms: Vec<Vec3> = vec![];
//     for (piece, entity) in pieces.iter() {
//         commands.entity(entity).remove::<Moving>();
//         for position_entity in piece.entities.iter() {
//             let t = positions
//                 .get_mut(*position_entity)
//                 .expect("Piece without position should not exist");
//             piece_transforms.push(t.translation);
//         }

//         // The issue was that the code expected pixel perfect placement.
//         // Add a 5% acceptance factor.
//         // We could put this in a method to clean up the code ?
//         let adjusted_min_x = board.min_x * 0.95;
//         let adjusted_min_y = board.min_y * 0.95;
//         let adjusted_max_x = board.max_x * 1.05;
//         let adjusted_max_y = board.max_y * 1.05;

//         let in_board = piece_transforms.iter().map(|t| t).all(|t| {
//             adjusted_min_x <= t.x
//                 && t.x <= adjusted_max_x
//                 && adjusted_min_y <= t.y
//                 && t.y <= adjusted_max_y
//         });

//         if in_board {
//             // TODO: we are once again iterating over the transform. This is not efficient.
//             for position_entity in piece.entities.iter() {
//                 let mut t = positions
//                     .get_mut(*position_entity)
//                     .expect("Piece without position should not exist");
//                 // Here we remove the modulo of a SQUARE height to map to a board position.
//                 let current_x_mod = (t.translation.x as i32) % SQUARE_WIDTH;
//                 let current_y_mod = (t.translation.y as i32) % SQUARE_WIDTH;
//                 let half_width = SQUARE_WIDTH / 2;
//                 if current_x_mod > half_width {
//                     (*t).translation.x =
//                         (t.translation.x as i32 - current_x_mod + SQUARE_WIDTH) as f32;
//                 } else {
//                     (*t).translation.x = (t.translation.x as i32 - current_x_mod) as f32;
//                 }

//                 if current_y_mod > half_width {
//                     (*t).translation.y =
//                         (t.translation.y as i32 - current_y_mod + SQUARE_WIDTH) as f32;
//                 } else {
//                     (*t).translation.y = (t.translation.y as i32 - current_y_mod) as f32;
//                 }
//                 // TODO: Save the board squares that are filled.
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd_ko() {
        // Given
        let piece_pos = Vec3::new(1.0, 1.0, 1.0);
        let current_pos = Vec2::new(60., 40.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn test_even_odd_same_position() {
        // Given
        let piece_pos = Vec3::new(1.0, 1.0, 1.0);
        let current_pos = Vec2::new(1., 1.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_odd_ok_different_position_in_area() {
        // Given
        let piece_pos = Vec3::new(1.0, 1.0, 1.0);
        let current_pos = Vec2::new(5., 10.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_odd_ok_left_side_in_area() {
        // Given
        let piece_pos = Vec3::new(10.0, 10.0, 1.0);
        let current_pos = Vec2::new(5., 5.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, true);
    }
}
