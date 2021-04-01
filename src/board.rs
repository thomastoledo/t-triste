use bevy::prelude::*;

use crate::shape::{Shape, ShapeBuilder};
use crate::position::Position;

// Plugins
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_board.system());
    }
}

// Components

// Cela va représenter un board
// * * * *
// * * * *
// * * * *
struct Board;

// Systems
fn spawn_board(
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    let material = materials.add(Color::rgb(0.60, 0.40, 0.).into());

    ShapeBuilder::new_board(commands, material, 300, 250, 5, 3);

    let board = Board {};

    commands.with(board);
}
