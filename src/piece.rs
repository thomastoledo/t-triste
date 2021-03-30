use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::math::Vec3;
use crate::shape::{Shape, ShapeBuilder};

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
#[derive(Default, Debug)]
struct Piece {
    rotation: f32,
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
    // See if we attach a Moving component in the piece for the query to avoid double loop
    mut query: Query<(&Piece, &mut Shape, &mut Transform)>,
) {
    for (piece, mut shape, mut transform) in query.iter_mut() {
        // For debug only
        // println!("piece = {:?}", piece);
        // println!("shape = {:?}", shape);
        // println!("transform = {:?}", transform);
        if piece.moving {
            for event in cursor_moved_event.iter() {
                // This also works
                // let (x, y) = <(f32, f32)>::from(event.position);
                let event_position = *event.position;
                let x = event_position.x;
                let y = event_position.y;
                // This is probably useless for now
                // shape.change_origin(x as i32, y as i32);
                // TODO: Only the last square inserted for the piece is moving
                *transform = Transform::from_translation(Vec3::new(x, y, 1.0));
            }
        }
    }
}

fn spawn_piece(
    mut materials: ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    // TODO: Use a position struct
    // let position = Position()

    let piece = Piece {
        moving: false,
        rotation: 0.,
    };

    let materials = materials.add(Color::rgb(0.68, 0.1, 1.03).into());
    ShapeBuilder::new_rectangle_piece(commands, materials, 200, 200);
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
