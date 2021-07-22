use std::vec;
use bevy::{math::vec3, prelude::*};
use t_triste_macro::PieceBehavior;

#[derive(PieceBehavior)]
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
