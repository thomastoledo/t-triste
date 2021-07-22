use bevy::prelude::*;
use std::vec;

use t_triste_macro::PieceBehavior;

use crate::piece::piece_builder::{PieceBuilder, SQUARE_WIDTH};

#[derive(PieceBehavior)]
pub struct Z {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
}

impl Z {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        let mut positions = vec![];
        positions.append(&mut PieceBuilder::new_horizontal_rectangle(
            start_x, start_y, 2, 1.,
        ));
        positions.append(&mut PieceBuilder::new_horizontal_rectangle(
            start_x + SQUARE_WIDTH,
            start_y + SQUARE_WIDTH,
            2,
            1.,
        ));

        Z {
            positions,
            color: Color::rgb(0.46, 0.98, 1.13),
            moving: false,
        }
    }
}
