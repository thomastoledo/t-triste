use std::{f32::consts::{FRAC_PI_2}, vec};

use bevy::{math::vec3, prelude::*};

use super::piece::Piece;

pub struct Square {
    positions: Vec<Vec3>,
    color: Color,
    moving: bool,
}

impl Square {
    pub fn new(start_x: i32, start_y: i32) -> Self {
        Square {
            positions: vec![vec3(start_x as f32, start_y as f32, 1.)],
            color: Color::rgb(0.01, 1.0, 0.42536772),
            moving: false,
        }
    }
}

impl Piece for Square {
    fn positions(&self) -> Vec<Vec3> {
        self.positions.clone()
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn rotate(&mut self) {
        let mut new_positions = vec![];

        let s:f32 = FRAC_PI_2.sin();
        let c:f32 = FRAC_PI_2.cos();

        // We can unwrap as the first position exist
        let first_pos = self.positions.first().unwrap();
        let cx = first_pos.x;
        let cy = first_pos.y;

        for position in self.positions.iter() {
            let trans_x = position.x - cx;
            let trans_y = position.y - cy;

            let xnew = trans_x * c - trans_y * s;
            let ynew = trans_x * s + trans_y * c;

            new_positions.push(Vec3::new(
                xnew + cx,
                ynew + cy,
                position.z
            ));
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
