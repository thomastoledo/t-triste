use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use bevy::prelude::*;

use crate::cursor::Cursor;
use crate::position::Position;
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
    cursor: Res<Cursor>,
    // See if we attach a Moving component in the piece for the query to avoid double loop
    shapes: Query<&Shape, With<Piece>>,
    mut positions: Query<(&mut Position, &mut Transform)>,
) {
    for shape in shapes.iter() {
        if cursor.is_pressed {
            let first_entity = shape.entities.first().unwrap();
            let first_transform = (*positions.get_mut(*first_entity).unwrap().1);
            (*positions.get_mut(*first_entity).unwrap().1) = Transform::from_xyz(
                cursor.current_pos.x,
                cursor.current_pos.y,
                1.0,
            );
            let delta_x = - first_transform.translation.x + cursor.current_pos.x;
            let delta_y = - first_transform.translation.y + cursor.current_pos.y;

            for entity in shape.entities.iter().skip(1) {
                let new_transform = (*positions.get_mut(*entity).unwrap().1);
                (*positions.get_mut(*entity).unwrap().1) = Transform::from_xyz(
                    new_transform.translation.x + delta_x,
                    new_transform.translation.y + delta_y,
                    1.0,
                );
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
    // TODO see if we can put it in the new_rectangle_piece.
    commands.with(piece);
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
