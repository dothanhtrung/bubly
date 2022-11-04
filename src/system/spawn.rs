use crate::component::{
    AnimationTimer, Bird, FinalResult, GameState, HighScore, MainMenu, Obstacle, Score, Scroll,
};

use bevy::asset::{AssetServer, Assets, Handle};
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::prelude::{
    default, AlignItems, ButtonBundle, Camera2dBundle, Color, Commands, DespawnRecursiveExt,
    Entity, JustifyContent, NodeBundle, Query, Res, ResMut, Size, SpriteSheetBundle, State, Style,
    Text, TextBundle, TextStyle, TextureAtlas, TextureAtlasSprite, Time, Timer, TimerMode,
    Transform, TransformBundle, UiRect, Val, Windows, With,
};
use bevy::sprite::SpriteBundle;
use bevy::ui::{BackgroundColor, FlexDirection};
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

const PLAYER_SIZE: Real = 56.;
const OBSTACLE_WIDTH: Real = 100.;
const OBSTACLE_HEIGHT: Real = 2000.;
const GROUND_WIDTH: Real = 2000.;
const GROUND_HEIGHT: Real = 100.;
const BG_WIDTH: Real = 1536.;
const HVELOC: Real = -90.;

pub fn flap_anim(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn spawn_scrollable(
    command: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
    z: f32,
    width: f32,
    hveloc: Real,
    img: &str,
) {
    command.spawn((
        Scroll { width },
        SpriteBundle {
            texture: asset_server.load(img),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Velocity {
            linvel: Vec2::new(hveloc, 0.),
            ..default()
        },
    ));
    command.spawn((
        Scroll { width },
        SpriteBundle {
            texture: asset_server.load(img),
            transform: Transform::from_xyz(x + width, y, z),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Velocity {
            linvel: Vec2::new(hveloc, 0.),
            ..default()
        },
    ));
}

fn spawn_bird(
    command: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bird.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(128., 128.), 1, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    command.spawn((
        Bird,
        RigidBody::Dynamic,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ActiveEvents::COLLISION_EVENTS,
        Collider::ball(PLAYER_SIZE),
        GravityScale(0.0),
        Velocity::default(),
    ));
}

pub fn setup(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    // Setup camera
    command.spawn(Camera2dBundle::default());

    spawn_bg(&mut command, &asset_server);

    // Spawn ground
    spawn_scrollable(
        &mut command,
        &asset_server,
        0.0,
        -window.height() / 2.0 + GROUND_HEIGHT / 2. - 20.,
        1.0,
        GROUND_WIDTH,
        HVELOC,
        "ground.png",
    );
    spawn_scrollable(
        &mut command,
        &asset_server,
        10.0,
        window.height() / 2.0 - GROUND_HEIGHT / 2. + 20.,
        1.0,
        GROUND_WIDTH,
        HVELOC,
        "ground.png",
    );
    // Ground collider
    command.spawn((
        Collider::cuboid(GROUND_WIDTH / 2., GROUND_HEIGHT / 2. - 6.),
        TransformBundle::from(Transform::from_xyz(
            0.,
            -window.height() / 2.0 + GROUND_HEIGHT / 2. - 20.,
            1.,
        )),
    ));
    command.spawn((
        Collider::cuboid(GROUND_WIDTH / 2., GROUND_HEIGHT / 2. - 6.),
        TransformBundle::from(Transform::from_xyz(
            0.,
            window.height() / 2.0 - GROUND_HEIGHT / 2. + 20.,
            1.,
        )),
    ));

    spawn_bird(&mut command, &asset_server, texture_atlases);
    spawn_ui(&mut command, &asset_server);
}

pub fn spawn_obstacle(
    mut command: Commands,
    windows: Res<Windows>,
    state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    if state.current() != &GameState::Running {
        return;
    }

    let window = windows.get_primary().unwrap();
    let mut rng = thread_rng();
    let pole_height = window.height() - PLAYER_SIZE * 4.35;
    let upper_height = rng.gen_range((GROUND_HEIGHT + 10.)..(pole_height - GROUND_HEIGHT - 10.));
    let lower_height = pole_height - upper_height;
    let pole_width = OBSTACLE_WIDTH / 2. - 6.;
    command
        .spawn((
            Obstacle { scored: false },
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2. + pole_width / 2., 0., 1.),
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            Velocity {
                linvel: Vec2::new(HVELOC, 0.),
                ..default()
            },
        ))
        .with_children(|child| {
            child.spawn((
                Collider::cuboid(pole_width, OBSTACLE_HEIGHT / 2.),
                SpriteBundle {
                    texture: asset_server.load("obstacle.png"),
                    transform: Transform::from_xyz(
                        0.,
                        window.height() / 2. - upper_height + OBSTACLE_HEIGHT / 2.,
                        0.,
                    ),
                    ..default()
                },
            ));
            child.spawn((
                Collider::cuboid(pole_width, OBSTACLE_HEIGHT / 2.),
                SpriteBundle {
                    texture: asset_server.load("obstacle.png"),
                    transform: Transform::from_xyz(
                        0.,
                        -window.height() / 2. + lower_height - OBSTACLE_HEIGHT / 2.,
                        0.,
                    ),
                    ..default()
                },
            ));
        });
}

pub fn drop_oor_obstacles(
    mut command: Commands,
    old_obj: Query<(Entity, &Transform), With<Obstacle>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    for (e, transform) in old_obj.iter() {
        if transform.translation.x + OBSTACLE_WIDTH / 2. < -window.width() {
            command.entity(e).despawn_recursive();
        }
    }
}

fn spawn_bg(command: &mut Commands, asset_server: &Res<AssetServer>) {
    command.spawn((SpriteBundle {
        texture: asset_server.load("bg1.png"),
        transform: Transform::from_xyz(0.0, 0., 0.0),
        ..default()
    },));

    spawn_scrollable(
        command,
        &asset_server,
        0.,
        0.,
        0.1,
        BG_WIDTH,
        -10.,
        "bg2.png",
    );
    spawn_scrollable(
        command,
        &asset_server,
        0.,
        0.,
        0.2,
        BG_WIDTH,
        -20.,
        "bg3.png",
    );
    spawn_scrollable(
        command,
        &asset_server,
        0.,
        0.,
        0.3,
        BG_WIDTH,
        -50.,
        "bg4.png",
    );
}

fn spawn_ui(command: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("Xolonium-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 69.0,
        color: Color::WHITE,
        ..default()
    };
    let highest_score_style = TextStyle {
        font,
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };

    command
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            ..default()
        })
        .with_children(|child| {
            child.spawn((
                TextBundle {
                    text: Text::from_section("0", text_style.clone()),
                    ..default()
                }
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Percent(80.),
                        top: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                }),
                Score,
            ));

            child
                .spawn((
                    MainMenu,
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Percent(100.0), Val::Percent(69.0)),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            MainMenu,
                            ButtonBundle {
                                style: Style {
                                    margin: UiRect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., -1.),
                                background_color: BackgroundColor(Color::NONE),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                FinalResult,
                                TextBundle::from_section("Score", text_style.clone()),
                            ));
                        });

                    parent
                        .spawn((
                            MainMenu,
                            ButtonBundle {
                                style: Style {
                                    margin: UiRect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., -1.),
                                background_color: BackgroundColor(Color::NONE),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                HighScore,
                                TextBundle::from_section("Highest", highest_score_style),
                            ));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.), Val::Px(65.)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Play", text_style));
                        });
                });
        });
}
