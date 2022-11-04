use crate::component::{Bird, GameState, MainMenu};
use bevy::input::mouse::MouseButton;
use bevy::input::Input;
use bevy::prelude::{Button, Changed, KeyCode, Query, Res, ResMut, State, Transform, With};
use bevy::ui::Interaction;
use bevy_rapier2d::dynamics::{GravityScale, Velocity};

pub fn start_game(
    kb_input: Res<Input<KeyCode>>,
    mut head_grav: Query<&mut GravityScale, With<Bird>>,
    mut state: ResMut<State<GameState>>,
    btn_interact: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mouse: Res<Input<MouseButton>>,
    menu_transform: Query<&mut Transform, With<MainMenu>>,
) {
    let mut play_btn_clicked = false;
    for interact in btn_interact.iter() {
        if *interact == Interaction::Clicked {
            play_btn_clicked = true;
        }
    }
    if kb_input.pressed(KeyCode::Space) || play_btn_clicked || mouse.just_pressed(MouseButton::Left)
    {
        hide_menu(menu_transform);

        if state.current() != &GameState::Running {
            state
                .set(GameState::Running)
                .expect("Cannot change state to Running");
        }
        for mut grav in head_grav.iter_mut() {
            grav.0 = 15.;
        }
    }
}

pub fn jump(
    kb_input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut bird_velocities: Query<(&mut Transform, &mut Velocity), With<Bird>>,
) {
    if kb_input.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        for (mut t, mut v) in bird_velocities.iter_mut() {
            v.linvel.y = 600.0;
            v.linvel.x = 0.0;
            t.rotation.z = 0.3;
        }
    }
}

fn hide_menu(mut menu_transform: Query<&mut Transform, With<MainMenu>>) {
    for mut transform in &mut menu_transform {
        transform.translation.z = -1.;
    }
}
