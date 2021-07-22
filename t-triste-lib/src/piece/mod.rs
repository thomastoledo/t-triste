pub mod board;

mod corner;
mod l;
mod piece;
mod piece_builder;
mod rectangle;
mod square;
mod z;

extern crate t_triste_macro;

use bevy::{math::vec3, prelude::*};

use crate::{
    cursor::Cursor,
    piece::{corner::Corner, l::L, rectangle::Rectangle, square::Square, z::Z},
};
use piece::{Piece, Position};
use piece_builder::SQUARE_WIDTH;

// Plugins
pub struct PiecePlugin;
struct GameState(Vec<Box<dyn Piece>>);

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_non_send_resource(GameState(vec![
            Box::new(Rectangle::new(100, 100)),
            Box::new(L::new(200, 300)),
            Box::new(Z::new(400, 500)),
            Box::new(Corner::new(100, 300)),
            Box::new(Square::new(300, 100)),
        ]))
        .add_system_to_stage(CoreStage::PreUpdate, clear.system())
        .add_system(release_piece.system())
        .add_system(click_piece.system())
        .add_system(move_piece.system())
        .add_system(draw_piece.system());
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
