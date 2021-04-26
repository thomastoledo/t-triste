use bevy::prelude::*;

use crate::piece_builder::PieceBuilder;

// Plugins
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_board.system());
    }
}

// Components

// This represent a board. For now the size is fixed
// * * * *
// * * * *
// * * * *
pub struct Board {
    pub entities: Vec<Entity>,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

// Systems
fn spawn_board(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let material = materials.add(Color::rgb(0.60, 0.40, 0.).into());

    PieceBuilder::new_board(&mut commands, material, 300, 250, 5, 3);
}
