use bevy::prelude::*;
mod piece;

// Start function
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(piece::PiecePlugin)
        .run();
}
