use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Actions>()
            .add_system(set_movement_actions.system());
    }
}

#[derive(Debug, Default)]
pub struct Actions {
    pub player_left_move: i8,
    pub player_right_move: i8,
    pub restart_requested: bool,
}

/// Queries actions every frame (allows navigation etc in the menu)
fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    actions.player_left_move = 0;
    actions.player_right_move = 0;

    if keyboard_input.pressed(KeyCode::A) {
        actions.player_left_move -= 1;
    }

    if keyboard_input.pressed(KeyCode::D) {
        actions.player_left_move += 1;
    }

    if keyboard_input.pressed(KeyCode::J) {
        actions.player_right_move -= 1;
    }

    if keyboard_input.pressed(KeyCode::L) {
        actions.player_right_move += 1;
    }

    actions.restart_requested = keyboard_input.just_pressed(KeyCode::Space);
}
