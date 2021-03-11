use bevy::prelude::*;

use crate::position::Shape;

// Plugins
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_board.system());
    }
}

// Components

// Cela va représenter un board
// * * * *
// * * * *
// * * * *
struct Board {}

// Systems
fn spawn_board(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    let texture_handle = asset_server.load("square.png");
    let material = materials.add(texture_handle.into());

    // TODO: Proper builder ?
    let mut shape = Shape::new();
    shape.new_horizontal_rectangle(400, 300, 5);
    shape.new_horizontal_rectangle(400, 350, 5);
    shape.new_horizontal_rectangle(400, 400, 5);

    let board = Board {};

    for square in &shape.squares {
        commands.spawn(
            SpriteBundle {
                material: material.clone(),
                transform: Transform::from_translation(square.to_vec()),
                ..Default::default()
            }
        );
    }

    commands
            .with(board)
            .with(shape);
}
