use bevy::math::Vec2;

use crate::piece_builder::SQUARE_WIDTH;

trait Rotate {
    fn rotate(&mut self);
}

struct Rect {
    positions: Vec<Vec2>,
}

impl Rect {
    pub fn new(positions: Vec<Vec2>) -> Self {
        if positions.is_empty() {
            panic!("WTF, insert positions please")
        }

        let rec = Rect { positions };
        if !rec.is_horizontal() && !rec.is_vertical() {
            panic!("WTF, this is not a rectangle");
        }
        rec
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

impl Rotate for Rect {
    fn rotate(&mut self) {
        let position_length = self.positions.len();
        let middle_index = position_length / 2;
        let central_piece_position = self.positions[middle_index];

        let compute_delta = | idx |  (idx as f32 - middle_index as f32) * SQUARE_WIDTH as f32;

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
}

#[cfg(test)]
mod tests {
    use bevy::math::vec2;

    use super::*;

    #[test]
    fn test_rotate_90() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(200., 50.), vec2(200., 100.), vec2(200., 150.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(250., 100.), vec2(200., 100.), vec2(150., 100.)]
        );
    }

    #[test]
    fn test_rotate_180() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(250., 100.), vec2(200., 100.), vec2(150., 100.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(200., 150.), vec2(200., 100.), vec2(200., 50.)]
        );
    }

    #[test]
    fn test_rotate_270() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(200., 150.), vec2(200., 100.), vec2(200., 50.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(250., 100.), vec2(200., 100.), vec2(150., 100.)]
        );
    }
    
    #[test]
    fn test_rotate_360() {
        // Given
        let mut rectangle = Rect::new(vec![vec2(150., 100.), vec2(200., 100.), vec2(250., 100.)]);

        // When
        rectangle.rotate();

        // Then
        assert_eq!(
            rectangle.positions,
            vec![vec2(200., 150.), vec2(200., 100.), vec2(200., 50.)]
        );
    }
}
