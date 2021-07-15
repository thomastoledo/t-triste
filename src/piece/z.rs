use std::{f32::consts::FRAC_PI_2, vec};

use bevy::prelude::*;

use crate::piece::piece_builder::{PieceBuilder, SQUARE_WIDTH};

use super::piece::Piece;

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
            start_x + SQUARE_WIDTH, start_y + SQUARE_WIDTH, 2, 1.,
        ));

        Z {
            positions,
            color: Color::rgb(0.46, 0.98, 1.13),
            moving: false,
        }
    }
}

impl Piece for Z {
    fn positions(&self) -> Vec<Vec3> {
        self.positions.clone()
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn rotate(&mut self) {
        let mut new_positions = vec![];

        let s: f32 = FRAC_PI_2.sin();
        let c: f32 = FRAC_PI_2.cos();

        // We can unwrap as the first position exist
        let first_pos = self.positions.first().unwrap();
        let cx = first_pos.x;
        let cy = first_pos.y;

        for position in self.positions.iter() {
            let trans_x = position.x - cx;
            let trans_y = position.y - cy;

            let xnew = trans_x * c - trans_y * s;
            let ynew = trans_x * s + trans_y * c;

            new_positions.push(Vec3::new(xnew + cx, ynew + cy, position.z));
        }

        self.positions = new_positions;
    }

    fn move_it(&mut self, cursor: &Res<crate::cursor::Cursor>) {
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

    fn set_moving(&mut self, moving: bool) {
        self.moving = moving;
    }

    fn is_moving(&self) -> bool {
        self.moving
    }
}
