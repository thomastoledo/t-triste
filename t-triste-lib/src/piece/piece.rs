use bevy::prelude::*;

use crate::cursor::Cursor;

use crate::SQUARE_WIDTH;

// Components
pub struct Position;

pub trait Piece {
    fn positions(&self) -> Vec<Vec3>;
    fn color(&self) -> Color;
    fn rotate(&mut self);
    fn snap(&mut self);
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

// #[cfg(test)]
// mod tests {
//     // use super::*;
//     // #[test]
//     // fn test_even_odd_ko() {
//     //     // Given
//     //     let piece_pos = Vec3::new(1.0, 1.0, 1.0);
//     //     let current_pos = Vec2::new(60., 40.);
//     //     // When
//     //     let result = Piece::is_even_odd(piece_pos, current_pos);
//     //     // Then
//     //     assert_eq!(result, false);
//     // }

//     // #[test]
//     // fn test_even_odd_same_position() {
//     //     // Given
//     //     let piece_pos = Vec3::new(1.0, 1.0, 1.0);
//     //     let current_pos = Vec2::new(1., 1.);

//     //     // When
//     //     let result = Piece::is_even_odd(piece_pos, current_pos);

//     //     // Then
//     //     assert_eq!(result, true);
//     // }

//     // #[test]
//     // fn test_even_odd_ok_different_position_in_area() {
//     //     // Given
//     //     let piece_pos = Vec3::new(1.0, 1.0, 1.0);
//     //     let current_pos = Vec2::new(5., 10.);

//     //     // When
//     //     let result = Piece::is_even_odd(piece_pos, current_pos);

//     //     // Then
//     //     assert_eq!(result, true);
//     // }

//     #[test]
//     fn test_even_odd_ok_left_side_in_area() {
//         // Given
//         let piece_pos = Vec3::new(10.0, 10.0, 1.0);
//         let current_pos = Vec2::new(5., 5.);

//         // When
//         let result = Piece::is_even_odd(piece_pos, current_pos);

//         // Then
//         assert_eq!(result, true);
//     }
// }
