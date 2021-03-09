use bevy::prelude::*;
use t_triste::*;

// Start function
fn main() {
    App::build()
        .add_plugin(GamePlugin)
        .run();
}
