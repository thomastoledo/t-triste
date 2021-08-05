use bevy::prelude::*;
use std::vec;

use t_triste_macro::PieceBehavior;

use crate::piece::{SQUARE_WIDTH, piece_builder::{PieceBuilder}};

#[derive(PieceBehavior)]
pub struct Corner {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
}

impl Corner {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        let mut positions = vec![];

        positions.append(&mut PieceBuilder::new_horizontal_rectangle(
            start_x, start_y, 2, 1.,
        ));
        positions.append(&mut PieceBuilder::new_horizontal_rectangle(
            start_x,
            start_y + SQUARE_WIDTH,
            1,
            1.0,
        ));
        Corner {
            positions,
            color: Color::rgb(0.83, 1.02, 0.18),
            moving: false,
        }
    }
}
