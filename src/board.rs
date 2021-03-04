use bevy::prelude::*;

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
    let texture_handle = asset_server.load("board.png");
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        }).with(Board {});
}


