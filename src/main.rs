mod component;
mod system;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::WindowPlugin;
use bevy_rapier2d::prelude::*;

use crate::component::{GameData, GameOverEvent, GameState};
use system::infinitive_ground;
use system::input;
use system::spawn::{drop_oor_obstacles, flap_anim, setup, spawn_obstacle};
use system::*;

fn main() {
    App::new()
        .insert_resource(GameData::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Bubly".to_string(),
                        width: 720.,
                        height: 1280.,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(40.0))
        .add_event::<GameOverEvent>()
        .add_startup_system(setup)
        .add_system(flap_anim)
        .add_system(infinitive_ground)
        .add_system_set(SystemSet::on_update(GameState::Waiting).with_system(input::start_game))
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(bird_crash)
                .with_system(score)
                .with_system(input::jump)
                .with_system(show_menu.after(bird_crash))
                .with_system(birdhead_direction),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_run_criteria(FixedTimestep::step(5.))
                .with_system(spawn_obstacle)
                .with_system(drop_oor_obstacles),
        )
        .add_state(GameState::Waiting)
        .run();
}
