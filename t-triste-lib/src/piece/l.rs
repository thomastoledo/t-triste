use bevy::prelude::*;
use std::vec;

use t_triste_macro::PieceBehavior;

use crate::piece::piece_builder::{PieceBuilder, SQUARE_WIDTH};

#[derive(PieceBehavior)]
pub struct L {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
}

impl L {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        let mut positions = vec![];
        positions.append(&mut PieceBuilder::new_horizontal_rectangle(
            start_x, start_y, 1, 1.,
        ));
        positions.append(&mut PieceBuilder::new_horizontal_rectangle(
            start_x + SQUARE_WIDTH,
            start_y,
            1,
            1.,
        ));
        for i in 1..3 {
            positions.append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + (i * SQUARE_WIDTH),
                1,
                1.,
            ));
        }
        L {
            positions,
            color: Color::rgb(1.56, 0.12, 0.03),
            moving: false,
        }
    }
}
