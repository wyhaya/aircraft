use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 1000;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: env!("CARGO_PKG_NAME").to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(fps_update)
        .add_system(score_update)
        .add_system(aircraft_move)
        .add_system(bullet_create)
        .add_resource(BulletCreateTimer(Timer::new(
            Duration::from_millis(100),
            true,
        )))
        .add_system(bullet_move)
        .add_resource(BulletMoveTimer(Timer::new(Duration::from_millis(20), true)))
        .add_system(obstacle_create)
        .add_resource(ObstacleCreateTimer(Timer::new(
            Duration::from_millis(1000),
            true,
        )))
        .add_system(obstacle_move)
        .add_resource(ObstacleMoveTimer(Timer::new(
            Duration::from_millis(30),
            true,
        )))
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
    distance: f32,
}

impl Aircraft {
    const WIDTH: f32 = 67.2;
    const HEIGHT: f32 = 80.0;
    fn vec2() -> Vec2 {
        Vec2::new(Self::WIDTH, Self::HEIGHT)
    }
}

struct Bullet;

impl Bullet {
    const WIDTH: f32 = 16.25;
    const HEIGHT: f32 = 40.0;
    fn vec2() -> Vec2 {
        Vec2::new(Self::WIDTH, Self::HEIGHT)
    }
}

struct BulletCreateTimer(Timer);
struct BulletMoveTimer(Timer);

struct Obstacle;

impl Obstacle {
    const SIZE: f32 = 60.;
    fn vec2() -> Vec2 {
        Vec2::splat(Self::SIZE)
    }
}

struct ObstacleCreateTimer(Timer);
struct ObstacleMoveTimer(Timer);

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
    let aircraft_y = -((WINDOW_HEIGHT as f32 / 2.0 - Aircraft::HEIGHT * 2.0) as f32);

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
            sprite: Sprite::new(Aircraft::vec2()),
            ..Default::default()
        })
        .with(Aircraft { distance: 10. })
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

fn aircraft_move(input: Res<Input<KeyCode>>, mut query: Query<(&Aircraft, &mut Transform)>) {
    for (aircraft, mut transform) in query.iter_mut() {
        if input.pressed(KeyCode::Up) {
            let max = (WINDOW_HEIGHT as f32 / 2.0) - Aircraft::HEIGHT / 2.0;
            if max > transform.translation.y {
                transform.translation.y += aircraft.distance;
            }
        }
        if input.pressed(KeyCode::Down) {
            let max = -(WINDOW_HEIGHT as f32 / 2.0) + Aircraft::HEIGHT / 2.0;
            if max < transform.translation.y {
                transform.translation.y -= aircraft.distance;
            }
        }
        if input.pressed(KeyCode::Left) {
            let max = -(WINDOW_WIDTH as f32 / 2.0) + Aircraft::WIDTH / 2.0;
            if max < transform.translation.x {
                transform.translation.x -= aircraft.distance;
            }
        }
        if input.pressed(KeyCode::Right) {
            let max = (WINDOW_WIDTH as f32 / 2.0) - Aircraft::WIDTH / 2.0;
            if max > transform.translation.x {
                transform.translation.x += aircraft.distance;
            }
        }
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
    mut query: Query<&mut Transform, With<Aircraft>>,
) {
    timer.0.tick(time.delta_seconds());
    if !timer.0.finished() {
        return;
    }
    if input.pressed(KeyCode::A) {
        for transform in query.iter_mut() {
            commands
                .spawn(SpriteBundle {
                    material: materials.bullet.clone(),
                    sprite: Sprite::new(Bullet::vec2()),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        0.,
                    )),
                    ..Default::default()
                })
                .with(Bullet);
            audio.play(sounds.shot.clone());
        }
    }
}

fn bullet_move(
    time: Res<Time>,
    mut timer: ResMut<BulletMoveTimer>,
    commands: &mut Commands,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    timer.0.tick(time.delta_seconds());
    if !timer.0.finished() {
        return;
    }
    let max = (WINDOW_HEIGHT as f32 / 2.0) - Bullet::HEIGHT / 2.0;
    for (ent, mut transform) in query.iter_mut() {
        transform.translation.y += 10.;
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
    let y = WINDOW_HEIGHT as f32 / 2.0 - Obstacle::SIZE;
    let mut x = fastrand::f32() * (WINDOW_WIDTH / 2) as f32;
    if !fastrand::bool() {
        x = -x;
    }
    commands
        .spawn(SpriteBundle {
            material: materials.obstacle.clone(),
            sprite: Sprite::new(Obstacle::vec2()),
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..Default::default()
        })
        .with(Obstacle);
}

fn obstacle_move(
    time: Res<Time>,
    mut timer: ResMut<ObstacleMoveTimer>,
    commands: &mut Commands,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut obstacle_query: Query<(Entity, &mut Transform), With<Obstacle>>,
    aircraft_query: Query<&Transform, With<Aircraft>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut score_query: Query<&mut Score>,
) {
    timer.0.tick(time.delta_seconds());
    if !timer.0.finished() {
        return;
    }

    let aircraft_position = aircraft_query.iter().next().unwrap().translation;
    let aircraft_size = Aircraft::vec2();
    let max = -((WINDOW_HEIGHT / 2) as f32) - -40.;

    'obstacle: for (obstacle_entity, mut transform) in obstacle_query.iter_mut() {
        transform.translation.y -= 3.;
        // The obstacle moves outside the window
        if max > transform.translation.y {
            commands.despawn(obstacle_entity);
            continue 'obstacle;
        }

        // Obstacle and aircraft collision
        let (obstacle_position, obstacle_size) = (transform.translation, Obstacle::vec2());
        if collide(
            aircraft_position,
            aircraft_size,
            obstacle_position,
            obstacle_size,
        )
        .is_some()
        {
            score_query.iter_mut().next().unwrap().collision();
            commands.despawn(obstacle_entity);
            continue 'obstacle;
        }

        // Obstacle and bullet collision
        for (bullet_entiry, transform) in bullet_query.iter() {
            let (bullet_position, bullet_size) = (transform.translation, Bullet::vec2());
            if collide(
                obstacle_position,
                obstacle_size,
                bullet_position,
                bullet_size,
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
