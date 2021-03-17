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
struct Board;

struct Materials {
    board_material: Handle<ColorMaterial>,
}

// Systems
fn spawn_board(
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    let material = materials.add(Color::rgb(0.60, 0.40, 0.).into());

    let mut shape = Shape::new()
        .new_horizontal_rectangle(300, 250, 5)
        .new_horizontal_rectangle(300, 300, 5)
        .new_horizontal_rectangle(300, 350, 5);

    let board = Board {};

    for square in &shape.squares {
        commands.spawn(
            SpriteBundle {
                material: material.clone(),
                sprite: Sprite::new(Vec2::new(49.0, 49.0)), // 50px -1 to add border
                transform: Transform::from_translation(square.to_vec()),
                ..Default::default()
            }
        );
    }

    commands
        .with(board)
        .with(shape);
}
