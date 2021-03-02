use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use bevy::prelude::*;

// Plugins
pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_piece.system())
            .add_system(mouse_click_system.system());
    }
}

// Components
#[derive(Default)]
struct Piece {
    rotation: f32,
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
        for (mut piece, mut transform) in query.iter_mut() {
            piece.rotate_piece();
            transform.rotation = Quat::from_rotation_z(piece.rotation);
        }
    }
}

fn spawn_piece(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    let texture_handle = asset_server.load("rectangle.png");
    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        }).with(Piece::default());
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
