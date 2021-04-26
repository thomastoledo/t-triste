use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use bevy::prelude::*;

use crate::cursor::Cursor;
use crate::piece_builder::PieceBuilder;
use crate::position::Position;
use crate::{board::Board, piece_builder::SQUARE_WIDTH};

// Plugins
pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_piece.system())
            .add_system(click_piece.system())
            .add_system(move_piece.system())
            .add_system(incrust_in_board.system());
    }
}

// Components
pub struct Moving;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Piece {
    pub entities: Vec<Entity>,
    pub rotation: f32,
}

impl Piece {
    // Rotate a piece by 90° in radians
    // TODO: This does not work anymore.
    //  We need an algorithm that does the rotation (change origin of each square)
    fn rotate_piece(&mut self) {
        let next_rad = self.rotation + FRAC_PI_2;
        if next_rad == (2.0 * PI) {
            self.rotation = 0.0;
        } else {
            self.rotation = next_rad;
        }
    }

    /// Piece translation are in fact the middle of the piece
    fn is_even_odd(piece_pos: Vec3, current_pos: Vec2) -> bool {
        piece_pos.x - (SQUARE_WIDTH / 2) as f32 <= current_pos.x
            && current_pos.x <= piece_pos.x + (SQUARE_WIDTH / 2) as f32
            && piece_pos.y - (SQUARE_WIDTH / 2) as f32 <= current_pos.y
            && current_pos.y <= piece_pos.y + (SQUARE_WIDTH / 2) as f32
    }
}

// Systems
fn spawn_piece(mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
    let rectangle_material = materials.add(Color::rgb(0.68, 0.1, 1.03).into());
    PieceBuilder::new_rectangle_piece(&mut commands, rectangle_material, 200, 200);
    let l_material = materials.add(Color::rgb(1.56, 0.12, 0.03).into());
    PieceBuilder::new_l_piece(&mut commands, l_material, 600, 50);
    let z_material = materials.add(Color::rgb(0.46, 0.98, 1.13).into());
    PieceBuilder::new_z_piece(&mut commands, z_material, 100, 350);
    let corner_material = materials.add(Color::rgb(0.83, 1.02, 0.18).into());
    PieceBuilder::new_corner_piece(&mut commands, corner_material, 50, 350);
    let square_material = materials.add(Color::rgb(0.01, 1.0, 0.42536772).into());
    PieceBuilder::new_dot_square_piece(&mut commands, square_material, 400, 100);
}

fn click_piece(
    mut commands: Commands,
    cursor: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
    piece_query: Query<(&Piece, Entity)>,
    positions: Query<&Transform, With<Position>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (piece, entity) in piece_query.iter() {
            for position_entity in piece.entities.iter() {
                let trans = positions
                    .get(*position_entity)
                    .expect("Piece without pos should not exist");
                if Piece::is_even_odd(trans.translation, cursor.current_pos) {
                    commands.entity(entity).insert(Moving);
                }
            }
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        // TODO: See doc around rotation part
        // for (mut piece, mut transform) in pieces.iter_mut() {
        //     piece.rotate_piece();
        //     transform.rotation = Quat::from_rotation_z(piece.rotation);
        // }
    }
}

fn move_piece(
    cursor: Res<Cursor>,
    pieces: Query<&Piece, With<Moving>>,
    mut positions: Query<(&mut Position, &mut Transform)>,
) {
    if cursor.is_pressed {
        for piece in pieces.iter() {
            let first_entity = piece.entities.first().unwrap();
            let first_transform = *positions.get_mut(*first_entity).unwrap().1;
            (*positions.get_mut(*first_entity).unwrap().1) =
                Transform::from_xyz(cursor.current_pos.x, cursor.current_pos.y, 1.0);
            let delta_x = -first_transform.translation.x + cursor.current_pos.x;
            let delta_y = -first_transform.translation.y + cursor.current_pos.y;

            for entity in piece.entities.iter().skip(1) {
                let new_transform = *positions.get_mut(*entity).unwrap().1;
                (*positions.get_mut(*entity).unwrap().1) = Transform::from_xyz(
                    new_transform.translation.x + delta_x,
                    new_transform.translation.y + delta_y,
                    1.0,
                );
            }
        }
    }
}

fn incrust_in_board(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    boards: Query<&Board>,
    pieces: Query<(&Piece, Entity), With<Moving>>,
    mut positions: Query<&mut Transform, With<Position>>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    // We know for sure that we only have one board
    let board = boards.iter().next().unwrap();
    // TODO: algo to move each transform in the board.
    let mut piece_transforms: Vec<Vec3> = vec![];
    for (piece, entity) in pieces.iter() {
        commands.entity(entity).remove::<Moving>();
        for position_entity in piece.entities.iter() {
            let t = positions
                .get_mut(*position_entity)
                .expect("Piece without position should not exist");
            piece_transforms.push(t.translation);
        }

        // The issue here is that the code expect pixel perfect placement.
        // Add a 5% acceptance factor.
        // We could put this in a method to clean up the code ?
        let adjusted_min_x = board.min_x * 0.95;
        let adjusted_min_y = board.min_y * 0.95;
        let adjusted_max_x = board.max_x * 1.05;
        let adjusted_max_y = board.max_y * 1.05;

        let in_board = piece_transforms.iter().map(|t| t).all(|t| {
            adjusted_min_x <= t.x
                && t.x <= adjusted_max_x
                && adjusted_min_y <= t.y
                && t.y <= adjusted_max_y
        });
        println!(
            "Min x={:?} y={:?}, Max x={:?} y={:?}",
            board.min_x, board.min_y, board.max_x, board.max_y
        );
        println!("{:?}", piece_transforms);
        println!("{:?}", in_board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        use std::f32::consts::FRAC_PI_2;
        use std::f32::consts::PI;

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

    #[test]
    fn test_even_odd_ko() {
        // Given
        let piece_pos = Vec3::new(1.0, 1.0, 1.0);
        let current_pos = Vec2::new(60., 40.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn test_even_odd_same_position() {
        // Given
        let piece_pos = Vec3::new(1.0, 1.0, 1.0);
        let current_pos = Vec2::new(1., 1.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_odd_ok_different_position_in_area() {
        // Given
        let piece_pos = Vec3::new(1.0, 1.0, 1.0);
        let current_pos = Vec2::new(5., 10.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, true);
    }

    #[test]
    fn test_even_odd_ok_left_side_in_area() {
        // Given
        let piece_pos = Vec3::new(10.0, 10.0, 1.0);
        let current_pos = Vec2::new(5., 5.);

        // When
        let result = Piece::is_even_odd(piece_pos, current_pos);

        // Then
        assert_eq!(result, true);
    }
}
