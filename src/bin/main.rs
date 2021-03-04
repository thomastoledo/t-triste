use bevy::prelude::*;
use t_triste::*;

// Start function
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(piece::PiecePlugin)
        .add_plugin(board::BoardPlugin)
        .run();
}
