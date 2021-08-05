mod cursor;
mod piece;

use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;
use piece::{GameState, SQUARE_WIDTH, board::{self, Board}};

// Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WindowDescriptor {
            title: "T-Triste".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1., 0.90, 1.)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera.system())
        .add_plugin(cursor::CursorPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(piece::PiecePlugin)
        .add_system(incrust_in_board.system());
    }
}

// System
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle({
        let mut camera = OrthographicCameraBundle::new_2d();
        camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
        camera
    });
}

fn incrust_in_board(
    mut game_state: NonSendMut<GameState>,
    board: Option<Res<Board>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) || 
    board.is_none() {
        return;
    }

    let board = board.unwrap();

    // The issue was that the code expected pixel perfect placement.
    // Add a 5% acceptance factor.
    // We could put this in a method to clean up the code ?
    let adjusted_min_x = board.min_x * 0.95;
    let adjusted_min_y = board.min_y * 0.95;
    let adjusted_max_x = board.max_x * 1.05;
    let adjusted_max_y = board.max_y * 1.05;

    // We take the first moving piece
    // TODO: This could be improved
    let moving_piece_optional = game_state.0.iter_mut().find(|piece| piece.is_moving());
    if moving_piece_optional.is_none() {
        return;
    }
    let moving_piece = moving_piece_optional.unwrap();

    // TODO: Find the exact board position that is being filled
    let in_board = moving_piece.positions().iter().all(|t| {
        adjusted_min_x <= t.x
            && t.x <= adjusted_max_x
            && adjusted_min_y <= t.y
            && t.y <= adjusted_max_y
    });

    println!("BEFORE");
    println!("{:?}", &moving_piece.positions());
    if in_board {
        moving_piece.snap();
        // TODO: we are once again iterating over the transform. This is not efficient.
        // TODO: Save the board squares that are filled.
    }
    println!("{:?}", &moving_piece.positions());
}
