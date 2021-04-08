use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::math::const_vec2;
use bevy::prelude::*;
use std::time::Duration;

pub struct ObstaclePlugin;
pub struct Obstacle {
    speed: f32,
}
struct ObstacleTimer(Timer);

impl Obstacle {
    const WIDTH: f32 = 60.;
    const HEIGHT: f32 = 60.;
    pub const SIZE: Vec2 = const_vec2!([Self::WIDTH, Self::HEIGHT]);
}

struct ObstacleResource {
    material: Handle<ColorMaterial>,
}

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .insert_resource(ObstacleTimer(Timer::from_seconds(0.1, true)))
            .add_system(create.system())
            .add_system(update.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ObstacleResource {
        material: materials.add(asset_server.load("images/obstacle.png").into()),
    });
}

fn create(
    time: Res<Time>,
    mut timer: ResMut<ObstacleTimer>,
    mut commands: Commands,
    res: Res<ObstacleResource>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    timer
        .0
        .set_duration(Duration::from_secs_f32(fastrand::f32() * 3.));

    let y = WINDOW_HEIGHT / 2.0 + Obstacle::HEIGHT / 2.0;
    let mut x = fastrand::f32() * (WINDOW_WIDTH / 2. - Obstacle::WIDTH / 2.);
    if fastrand::bool() {
        x = -x;
    }
    commands
        .spawn_bundle(SpriteBundle {
            material: res.material.clone(),
            sprite: Sprite::new(Obstacle::SIZE),
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..Default::default()
        })
        .insert(Obstacle {
            speed: 100. + fastrand::f32() * 200.,
        });
}

fn update(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Obstacle, &mut Transform)>,
) {
    let min = -(WINDOW_HEIGHT / 2. + Obstacle::HEIGHT / 2.0);
    for (en, obstacle, mut transform) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * obstacle.speed;
        // The obstacle moves outside the window
        if min > transform.translation.y {
            commands.entity(en).despawn();
        }
    }
}
