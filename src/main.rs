mod background;

use background::BackgroundPlugin;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::math::const_vec2;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub const WINDOW_WIDTH: f32 = 640.;
pub const WINDOW_HEIGHT: f32 = 1000.;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: env!("CARGO_PKG_NAME").to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(BackgroundPlugin)
        .add_startup_system(setup.system())
        .add_system(fps_update.system())
        .add_system(score_update.system())
        .add_system(aircraft_move.system())
        .add_system(bullet_create.system())
        .add_resource(BulletCreateTimer(Timer::from_seconds(0.1, true)))
        .add_system(bullet_move.system())
        .add_system(obstacle_create.system())
        .add_resource(ObstacleCreateTimer(Timer::from_seconds(0.1, true)))
        .add_system(obstacle_move.system())
        .run();
}

struct Fps;

struct Score(i32);

impl Score {
    fn hit(&mut self) {
        self.0 += 1;
    }
    fn collision(&mut self) {
        self.0 -= 10;
    }
}

struct Aircraft {
    speed: f32,
}

impl Aircraft {
    const WIDTH: f32 = 67.2;
    const HEIGHT: f32 = 80.0;
    const SIZE: Vec2 = const_vec2!([Self::WIDTH, Self::HEIGHT]);
}

struct Bullet {
    speed: f32,
}

impl Bullet {
    const WIDTH: f32 = 16.25;
    const HEIGHT: f32 = 40.0;
    const SIZE: Vec2 = const_vec2!([Self::WIDTH, Self::HEIGHT]);
}

struct BulletCreateTimer(Timer);

struct Obstacle {
    speed: f32,
}

impl Obstacle {
    const WIDTH: f32 = 60.;
    const HEIGHT: f32 = 60.;
    const SIZE: Vec2 = const_vec2!([Self::WIDTH, Self::HEIGHT]);
}

struct ObstacleCreateTimer(Timer);

struct Materials {
    obstacle: Handle<ColorMaterial>,
    bullet: Handle<ColorMaterial>,
}

struct Sounds {
    shot: Handle<AudioSource>,
    explosion: Handle<AudioSource>,
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let aircraft = asset_server.load("images/aircraft.png");
    let obstacle = asset_server.load("images/obstacle.png");
    let bullet = asset_server.load("images/bullet.png");
    let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
    let aircraft_y = -(WINDOW_HEIGHT / 2.0 - Aircraft::HEIGHT * 2.0);

    commands
        .spawn(CameraUiBundle::default())
        .spawn(Camera2dBundle::default())
        // FPS
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(4.),
                    left: Val::Px(4.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                font: font.clone(),
                style: TextStyle {
                    font_size: 26.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Fps)
        // Score
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(4.),
                    right: Val::Px(4.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                font: font.clone(),
                style: TextStyle {
                    font_size: 26.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Score(0))
        // Aircraft
        .spawn(SpriteBundle {
            material: materials.add(aircraft.into()),
            transform: Transform::from_translation(Vec3::new(0., aircraft_y, 0.)),
            sprite: Sprite::new(Aircraft::SIZE),
            ..Default::default()
        })
        .with(Aircraft { speed: 500. })
        .insert_resource(Sounds {
            shot: asset_server.load("sounds/shot.mp3"),
            explosion: asset_server.load("sounds/explosion.mp3"),
        })
        .insert_resource(Materials {
            obstacle: materials.add(obstacle.into()),
            bullet: materials.add(bullet.into()),
        });
}

fn fps_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<Fps>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.0}", average);
            }
        }
    }
}

fn score_update(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in query.iter_mut() {
        text.value = format!("Score: {}", score.0);
    }
}

fn aircraft_move(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Aircraft, &mut Transform)>,
) {
    for (aircraft, mut transform) in query.iter_mut() {
        let max_x = WINDOW_WIDTH / 2.0 - Aircraft::WIDTH / 2.0;
        let max_y = WINDOW_HEIGHT / 2.0 - Aircraft::HEIGHT / 2.0;
        let distance = time.delta_seconds() * aircraft.speed;
        if input.pressed(KeyCode::Up) {
            transform.translation.y += distance;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += distance;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= distance;
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= distance;
        }
        transform.translation.y = transform.translation.y.min(max_y).max(-max_y);
        transform.translation.x = transform.translation.x.min(max_x).max(-max_x);
    }
}

fn bullet_create(
    time: Res<Time>,
    mut timer: ResMut<BulletCreateTimer>,
    commands: &mut Commands,
    input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    query: Query<&Transform, With<Aircraft>>,
) {
    timer.0.tick(time.delta_seconds());
    if !timer.0.finished() {
        return;
    }
    if input.pressed(KeyCode::A) {
        let transform = query.iter().next().unwrap();
        commands
            .spawn(SpriteBundle {
                material: materials.bullet.clone(),
                sprite: Sprite::new(Bullet::SIZE),
                transform: Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    0.,
                )),
                ..Default::default()
            })
            .with(Bullet { speed: 540. });
        audio.play(sounds.shot.clone());
    }
}

fn bullet_move(
    time: Res<Time>,
    commands: &mut Commands,
    mut query: Query<(Entity, &Bullet, &mut Transform)>,
) {
    let max = WINDOW_HEIGHT / 2.0 + Bullet::HEIGHT / 2.0;
    for (ent, bullet, mut transform) in query.iter_mut() {
        transform.translation.y += time.delta_seconds() * bullet.speed;
        if max < transform.translation.y {
            commands.despawn(ent);
        }
    }
}

fn obstacle_create(
    time: Res<Time>,
    mut timer: ResMut<ObstacleCreateTimer>,
    commands: &mut Commands,
    materials: Res<Materials>,
) {
    timer.0.tick(time.delta_seconds());
    if !timer.0.finished() {
        return;
    }
    timer.0.set_duration(fastrand::f32() * 3.);

    let y = WINDOW_HEIGHT / 2.0 + Obstacle::HEIGHT / 2.0;
    let mut x = fastrand::f32() * (WINDOW_WIDTH / 2. - Obstacle::WIDTH / 2.);
    if fastrand::bool() {
        x = -x;
    }
    commands
        .spawn(SpriteBundle {
            material: materials.obstacle.clone(),
            sprite: Sprite::new(Obstacle::SIZE),
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..Default::default()
        })
        .with(Obstacle {
            speed: 100. + fastrand::f32() * 200.,
        });
}

fn obstacle_move(
    time: Res<Time>,
    commands: &mut Commands,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut obstacle_query: Query<(Entity, &Obstacle, &mut Transform)>,
    aircraft_query: Query<&Transform, With<Aircraft>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut score_query: Query<&mut Score>,
) {
    let aircraft_position = aircraft_query.iter().next().unwrap().translation;
    let min = -(WINDOW_HEIGHT / 2. + Obstacle::HEIGHT / 2.0);

    'obstacle: for (obstacle_entity, obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * obstacle.speed;

        // The obstacle moves outside the window
        if min > transform.translation.y {
            commands.despawn(obstacle_entity);
            continue 'obstacle;
        }

        // Obstacle and aircraft collision
        let obstacle_position = transform.translation;
        if collide(
            aircraft_position,
            Aircraft::SIZE,
            obstacle_position,
            Obstacle::SIZE,
        )
        .is_some()
        {
            score_query.iter_mut().next().unwrap().collision();
            commands.despawn(obstacle_entity);
            continue 'obstacle;
        }

        // Obstacle and bullet collision
        for (bullet_entiry, transform) in bullet_query.iter() {
            if collide(
                obstacle_position,
                Obstacle::SIZE,
                transform.translation,
                Bullet::SIZE,
            )
            .is_some()
            {
                score_query.iter_mut().next().unwrap().hit();
                commands.despawn(obstacle_entity);
                commands.despawn(bullet_entiry);
                audio.play(sounds.explosion.clone());
                continue 'obstacle;
            }
        }
    }
}
