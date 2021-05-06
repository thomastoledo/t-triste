use bevy::{math::vec3, prelude::*};

use crate::piece_builder::SQUARE_WIDTH;
use crate::{cursor::Cursor, rectangle::Rectangle};

// Plugins
pub struct PiecePlugin;
struct GameState(Vec<Box<dyn Piece>>);

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_non_send_resource(GameState(vec![Box::new(Rectangle::new(100, 100))]))
            .add_system_to_stage(CoreStage::PreUpdate, clear.system())
            .add_system(release_piece.system())
            .add_system(click_piece.system())
            .add_system(move_piece.system())
            .add_system(draw_piece.system());
    }
}

// Components
pub struct Position;

pub trait Piece {
    fn positions(&self) -> Vec<Vec3>;
    fn color(&self) -> Color;
    fn rotate(&mut self);
    fn move_it(&mut self, cursor: &Res<Cursor>);
    fn set_moving(&mut self, moving: bool);
    fn is_moving(&self) -> bool;

    fn is_even_odd(&self, current_pos: Vec2) -> bool {
        self.positions().iter().any(|piece_pos| {
            piece_pos.x - (SQUARE_WIDTH / 2) as f32 <= current_pos.x
                && current_pos.x <= piece_pos.x + (SQUARE_WIDTH / 2) as f32
                && piece_pos.y - (SQUARE_WIDTH / 2) as f32 <= current_pos.y
                && current_pos.y <= piece_pos.y + (SQUARE_WIDTH / 2) as f32
        })
    }
}

// Systems
fn clear(mut commands: Commands, query: Query<Entity, With<Position>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn draw_piece(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: NonSendMut<GameState>,
) {
    for piece in game_state.0.iter_mut() {
        let material = materials.add(piece.color().into());
        let positions = piece.positions();
        for position in positions.iter() {
            commands
                .spawn_bundle(SpriteBundle {
                    material: material.clone(),
                    sprite: Sprite::new(Vec2::new(
                        (SQUARE_WIDTH - 1) as f32,
                        (SQUARE_WIDTH - 1) as f32,
                    )),
                    transform: Transform::from_translation(vec3(
                        position.x, position.y, position.z,
                    )),
                    ..Default::default()
                })
                .insert(Position);
        }
    }
}

fn move_piece(cursor: Res<Cursor>, mut game_state: NonSendMut<GameState>) {
    if cursor.is_pressed {
        game_state
            .0
            .iter_mut()
            .filter(|piece| piece.is_moving())
            .for_each(|piece| {
                piece.move_it(&cursor);
            })
    }
}

fn click_piece(
    cursor: Res<Cursor>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut game_state: NonSendMut<GameState>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for piece in game_state.0.iter_mut() {
            if piece.is_even_odd(cursor.current_pos) {
                piece.set_moving(true);
                return;
            }
        }
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        for piece in game_state.0.iter_mut() {
            if piece.is_even_odd(cursor.current_pos) {
                piece.rotate();
            }
        }
    }
}

fn release_piece(
    mouse_button_input: Res<Input<MouseButton>>,
    mut game_state: NonSendMut<GameState>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    game_state
        .0
        .iter_mut()
        .filter(|piece| piece.is_moving())
        .for_each(|piece| piece.set_moving(false));
}

// fn spawn_piece(mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
// let rectangle_material = materials.add(Color::rgb(0.68, 0.1, 1.03).into());
// PieceBuilder::new_rectangle_piece(&mut commands, rectangle_material, 200, 200);
// let l_material = materials.add(Color::rgb(1.56, 0.12, 0.03).into());
// PieceBuilder::new_l_piece(&mut commands, l_material, 600, 50);
// let z_material = materials.add(Color::rgb(0.46, 0.98, 1.13).into());
// PieceBuilder::new_z_piece(&mut commands, z_material, 100, 350);
// let corner_material = materials.add(Color::rgb(0.83, 1.02, 0.18).into());
// PieceBuilder::new_corner_piece(&mut commands, corner_material, 50, 350);
// let square_material = materials.add(Color::rgb(0.01, 1.0, 0.42536772).into());
// PieceBuilder::new_dot_square_piece(&mut commands, square_material, 400, 100);
// }

// fn incrust_in_board(
//     mut commands: Commands,
//     mouse_button_input: Res<Input<MouseButton>>,
//     boards: Query<&Board>,
//     pieces: Query<(&Piece, Entity), With<Moving>>,
//     mut positions: Query<&mut Transform, With<Position>>,
// ) {
//     if !mouse_button_input.just_released(MouseButton::Left) {
//         return;
//     }

//     // We know for sure that we only have one board
//     let board = boards.iter().next().unwrap();
//     // TODO: algo to move each transform in the board.
//     let mut piece_transforms: Vec<Vec3> = vec![];
//     for (piece, entity) in pieces.iter() {
//         commands.entity(entity).remove::<Moving>();
//         for position_entity in piece.entities.iter() {
//             let t = positions
//                 .get_mut(*position_entity)
//                 .expect("Piece without position should not exist");
//             piece_transforms.push(t.translation);
//         }

//         // The issue was that the code expected pixel perfect placement.
//         // Add a 5% acceptance factor.
//         // We could put this in a method to clean up the code ?
//         let adjusted_min_x = board.min_x * 0.95;
//         let adjusted_min_y = board.min_y * 0.95;
//         let adjusted_max_x = board.max_x * 1.05;
//         let adjusted_max_y = board.max_y * 1.05;

//         let in_board = piece_transforms.iter().map(|t| t).all(|t| {
//             adjusted_min_x <= t.x
//                 && t.x <= adjusted_max_x
//                 && adjusted_min_y <= t.y
//                 && t.y <= adjusted_max_y
//         });

//         if in_board {
//             // TODO: we are once again iterating over the transform. This is not efficient.
//             for position_entity in piece.entities.iter() {
//                 let mut t = positions
//                     .get_mut(*position_entity)
//                     .expect("Piece without position should not exist");
//                 // Here we remove the modulo of a SQUARE height to map to a board position.
//                 let current_x_mod = (t.translation.x as i32) % SQUARE_WIDTH;
//                 let current_y_mod = (t.translation.y as i32) % SQUARE_WIDTH;
//                 let half_width = SQUARE_WIDTH / 2;
//                 if current_x_mod > half_width {
//                     (*t).translation.x =
//                         (t.translation.x as i32 - current_x_mod + SQUARE_WIDTH) as f32;
//                 } else {
//                     (*t).translation.x = (t.translation.x as i32 - current_x_mod) as f32;
//                 }

//                 if current_y_mod > half_width {
//                     (*t).translation.y =
//                         (t.translation.y as i32 - current_y_mod + SQUARE_WIDTH) as f32;
//                 } else {
//                     (*t).translation.y = (t.translation.y as i32 - current_y_mod) as f32;
//                 }
//                 // TODO: Save the board squares that are filled.
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

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
