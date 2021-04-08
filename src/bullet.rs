use crate::{Aircraft, WINDOW_HEIGHT};
use bevy::math::const_vec2;
use bevy::prelude::*;

pub struct BulletPlugin;
pub struct Bullet {
    speed: f32,
}
struct BulletTimer(Timer);

struct BulletResource {
    shot: Handle<AudioSource>,
    material: Handle<ColorMaterial>,
}

impl Bullet {
    const WIDTH: f32 = 16.25;
    const HEIGHT: f32 = 40.0;
    pub const SIZE: Vec2 = const_vec2!([Self::WIDTH, Self::HEIGHT]);
}

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .insert_resource(BulletTimer(Timer::from_seconds(0.1, true)))
            .add_system(bullet_create.system())
            .add_system(bullet_move.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(BulletResource {
        shot: asset_server.load("sounds/shot.mp3"),
        material: materials.add(asset_server.load("images/bullet.png").into()),
    });
}

fn bullet_create(
    time: Res<Time>,
    mut timer: ResMut<BulletTimer>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    res: Res<BulletResource>,
    query: Query<&Transform, With<Aircraft>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    if input.pressed(KeyCode::A) {
        if let Ok(transform) = query.single() {
            commands
                .spawn_bundle(SpriteBundle {
                    material: res.material.clone(),
                    sprite: Sprite::new(Bullet::SIZE),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        0.,
                    )),
                    ..Default::default()
                })
                .insert(Bullet { speed: 540. });
            audio.play(res.shot.clone());
        }
    }
}

fn bullet_move(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Bullet, &mut Transform)>,
) {
    let max = WINDOW_HEIGHT / 2.0 + Bullet::HEIGHT / 2.0;
    for (ent, bullet, mut transform) in query.iter_mut() {
        transform.translation.y += time.delta_seconds() * bullet.speed;
        if max < transform.translation.y {
            commands.entity(ent).despawn();
        }
    }
}
