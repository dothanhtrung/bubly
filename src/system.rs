pub mod input;
pub mod spawn;

use bevy::prelude::*;
use bevy::text::Text;
use bevy_rapier2d::prelude::*;

use super::component::*;

pub fn bird_crash(
    mut contact_events: EventReader<CollisionEvent>,
    mut command: Commands,
    entities: Query<Entity, With<Obstacle>>,
    mut state: ResMut<State<GameState>>,
    mut score_text: Query<&mut Text, With<Score>>,
    mut bird: Query<(&mut Transform, &mut GravityScale, &mut Velocity)>,
    mut gamedata: ResMut<GameData>,
    mut gameover_writer: EventWriter<GameOverEvent>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            CollisionEvent::Started(_, _, _) => {
                gameover_writer.send(GameOverEvent);
                if state.current() != &GameState::Waiting {
                    state
                        .set(GameState::Waiting)
                        .expect("Cannot change state to Waiting");
                }

                // Remove all obstacles
                for e in entities.iter() {
                    command.entity(e).despawn_recursive();
                }

                // Reset bird position
                for (mut transform, mut grav, mut v) in &mut bird {
                    grav.0 = 0.;
                    v.linvel = Vec2::new(0., 0.);
                    v.angvel = 0.;
                    transform.translation.x = 0.;
                    transform.translation.y = 0.;
                    transform.rotation.z = 0.;
                }

                for mut text in &mut score_text {
                    gamedata.score = text.sections[0].value.parse().unwrap();
                    if gamedata.highest_score < gamedata.score {
                        gamedata.highest_score = gamedata.score;
                    }

                    text.sections[0].value = "0".to_string();
                }

                break;
            }
            _ => {}
        }
    }
}

pub fn score(
    mut texts: Query<&mut Text, With<Score>>,
    mut obstacles: Query<(&mut Obstacle, &Transform)>,
) {
    for mut text in &mut texts {
        let mut score: u64 = text.sections[0].value.parse().unwrap();
        for (mut obstacle, transform) in obstacles.iter_mut() {
            if !obstacle.scored && transform.translation.x < 0. {
                obstacle.scored = true;
                score += 1;
            }
        }
        text.sections[0].value = score.to_string();
    }
}

pub fn show_menu(
    gamedata: ResMut<GameData>,
    mut final_result_text: Query<&mut Text, (With<FinalResult>, Without<HighScore>)>,
    mut high_score_text: Query<&mut Text, (With<HighScore>, Without<FinalResult>)>,
    mut menu_transform: Query<&mut Transform, With<MainMenu>>,
    mut reader: EventReader<GameOverEvent>,
) {
    if reader.iter().next().is_some() {
        for mut transform in &mut menu_transform {
            transform.translation.z = 10.;
        }
        for mut text in &mut final_result_text {
            text.sections[0].value = gamedata.score.to_string();
        }
        for mut text in &mut high_score_text {
            text.sections[0].value = format!("Highest: {}", gamedata.highest_score);
        }
    }
}

pub fn infinitive_ground(windows: Res<Windows>, mut scrolls: Query<(&mut Transform, &Scroll)>) {
    let window = windows.get_primary().unwrap();
    let h_window_width = window.width() / 2.;

    for (mut transform, scroll) in scrolls.iter_mut() {
        if transform.translation.x + scroll.width / 2. <= -h_window_width {
            transform.translation.x += scroll.width * 3. / 2.;
        }
    }
}

pub fn birdhead_direction(mut tv: Query<(&mut Transform, &Velocity), With<Bird>>) {
    for (mut t, v) in tv.iter_mut() {
        if v.linvel.y < 0. && t.rotation.z > -0.5 {
            t.rotate_z(-0.1);
        }
    }
}
