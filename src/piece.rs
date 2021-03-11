use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use bevy::prelude::*;

// Plugins
pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_piece.system())
            .add_system(mouse_click_system.system())
            .add_system(mouse_move_system.system());
    }
}

// Components
#[derive(Default)]
struct Piece {
    rotation: f32,
    x: f32,
    y: f32,
    moving: bool,
}

impl Piece {
    // Rotate a piece by 90° in radians
    fn rotate_piece(&mut self) {
        let next_rad = self.rotation + FRAC_PI_2;
        if next_rad == (2.0 * PI) {
            self.rotation = 0.0;
        } else {
            self.rotation = next_rad;
        }
    }
}

// Systems
// This system prints messages when you press or release the left mouse button:
fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Piece, &mut Transform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (mut piece, _) in query.iter_mut() {
            piece.moving = true
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        for (mut piece, _) in query.iter_mut() {
            piece.moving = false
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        for (mut piece, mut transform) in query.iter_mut() {
            piece.rotate_piece();
            transform.rotation = Quat::from_rotation_z(piece.rotation);
        }
    }
}

fn mouse_move_system(
    mut cursor_moved_event: EventReader<CursorMoved>,
    // mut mouse_motion_events: EventReader<MouseMotion>,
    // See if we attach a Moving component in the piece for the query to avoid double loop
    mut query: Query<(&mut Piece, &mut Transform)>,
) {
    for (mut piece, mut transform) in query.iter_mut() {
        if piece.moving {
            for event in cursor_moved_event.iter() {
                // This also works
                // let (x, y) = <(f32, f32)>::from(event.position);
                let event_position = *event.position;
                let x = event_position.x;
                let y = event_position.y;
                // This is probably useless for now
                piece.x = x;
                piece.y = y;
                *transform = Transform::from_xyz(x, y, 1.0);
            }
        }
    }
}

fn spawn_piece(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    let piece = Piece {
        moving: false,
        rotation: 0.,
        x: 400.,
        y: 300.
    };
    let texture_handle = asset_server.load("rectangle.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_xyz(piece.x, piece.y, 1.0),
            ..Default::default()
        }).with(piece);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        use std::f32::consts::PI;
        use std::f32::consts::FRAC_PI_2;

        let mut piece = Piece::default();
        piece.rotate_piece();
        assert_eq!(piece.rotation, FRAC_PI_2);
        piece.rotate_piece();
        assert_eq!(piece.rotation, PI);
        piece.rotate_piece();
        assert_eq!(piece.rotation, 3.0 * FRAC_PI_2);
        piece.rotate_piece();
        assert_eq!(piece.rotation, 0.0);
    }
}
