use bevy::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(Cursor {
                current_pos: Vec2::default(),
                last_click_pos: Vec2::default(),
                is_pressed: false
            })
            .add_system_to_stage(CoreStage::PreUpdate, cursor_state.system());
    }
}

pub struct Cursor {
    pub current_pos: Vec2,
    pub last_click_pos: Vec2,
    pub is_pressed: bool
}

fn cursor_state(
    mut cursor_moved_event: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor: ResMut<Cursor>,
) {
    for event in cursor_moved_event.iter() {
        cursor.current_pos = event.position;

        if mouse_button_input.just_pressed(MouseButton::Left) {
            cursor.last_click_pos = event.position;
            cursor.is_pressed = true;
        }

        if mouse_button_input.just_released(MouseButton::Left) {
            cursor.is_pressed = false;
        }
    }
}
