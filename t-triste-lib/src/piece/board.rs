use bevy::prelude::*;

use crate::piece::SQUARE_WIDTH;

use super::piece_builder::PieceBuilder;

// Plugins
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Board::new(300, 250))
            .add_startup_system(draw_board.system());
    }
}

// Components

// Marker component
struct BoardPosition;

// This represent a board. For now the size is fixed
// * * * *
// * * * *
// * * * *
pub struct Board {
    pub positions: Vec<Vec3>,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    // true if the position is filled
    // vec[bool[]] ?
}

impl Board {
    fn new(start_x: i32, start_y: i32) -> Self {
        let nb_rows = 3;
        let nb_cols = 5;
        let mut positions = vec![];
        for i in 0..nb_rows {
            positions.append(&mut PieceBuilder::new_horizontal_rectangle(
                start_x,
                start_y + (i * SQUARE_WIDTH),
                nb_cols,
                0.,
            ));
        }
        Board {
            positions,
            min_x: start_x as f32,
            min_y: start_y as f32,
            max_x: (start_x + (nb_cols * SQUARE_WIDTH)) as f32,
            max_y: (start_y + (nb_rows * SQUARE_WIDTH)) as f32,
        }
    }
}

// Systems
fn draw_board(
    board: Res<Board>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let material = materials.add(Color::rgb(0.60, 0.40, 0.).into());
    board
        .positions
        .iter()
        .for_each(|position| {
            commands
                .spawn_bundle(SpriteBundle {
                    material: material.clone(),
                    sprite: Sprite::new(Vec2::new(
                        (SQUARE_WIDTH - 1) as f32,
                        (SQUARE_WIDTH - 1) as f32,
                    )),
                    transform: Transform::from_translation(*position),
                    ..Default::default()
                })
                .insert(BoardPosition);
        });
}
