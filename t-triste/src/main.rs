extern crate t_triste_lib;
use t_triste_lib::*;

use bevy::prelude::*;

// Start function
fn main() {
    App::build()
        .add_plugin(GamePlugin)
        .run();
}
