use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;

mod piece;
mod board;
mod position;
mod piece_builder;
mod cursor;

// Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(WindowDescriptor {
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
            .add_plugin(piece::PiecePlugin)
            .add_plugin(board::BoardPlugin);
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
