use crate::{Aircraft, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::math::const_vec2;
use bevy::prelude::*;

pub struct ShotPlugin;

struct BulletTimer(Timer);

pub struct Bullet {
    speed: f32,
}

impl Bullet {
    const WIDTH: f32 = 16.25;
    const HEIGHT: f32 = 40.0;
    pub const SIZE: Vec2 = const_vec2!([Self::WIDTH, Self::HEIGHT]);
}

pub struct Bubble {
    speed: Vec3,
}

impl Bubble {
    const HALF: f32 = 40.0 / 2.;
    pub const SIZE: Vec2 = const_vec2!([Self::HALF * 2., Self::HALF * 2.]);
}

struct AudioResources {
    bullet: Handle<AudioSource>,
}

struct MaterialResource {
    bullet: Handle<ColorMaterial>,
    bubble: Vec<Handle<ColorMaterial>>,
}

impl Plugin for ShotPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .insert_resource(BulletTimer(Timer::from_seconds(0.1, true)))
            .add_system(bullet_create.system())
            .add_system(bullet_move.system())
            .add_system(bubble_create.system())
            .add_system(bubble_move.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(AudioResources {
        bullet: asset_server.load("sounds/bullet.mp3"),
    });
    commands.insert_resource(MaterialResource {
        bullet: materials.add(asset_server.load("images/shot/bullet.png").into()),
        bubble: vec![
            materials.add(asset_server.load("images/shot/bubble1.png").into()),
            materials.add(asset_server.load("images/shot/bubble2.png").into()),
            materials.add(asset_server.load("images/shot/bubble3.png").into()),
        ],
    });
}

fn bullet_create(
    time: Res<Time>,
    mut timer: ResMut<BulletTimer>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    audio: Res<Audio>,
    audio_res: Res<AudioResources>,
    material_res: Res<MaterialResource>,
    query: Query<&Transform, With<Aircraft>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    if input.pressed(KeyCode::A) {
        let transform = query.single().unwrap();
        commands
            .spawn_bundle(SpriteBundle {
                material: material_res.bullet.clone(),
                sprite: Sprite::new(Bullet::SIZE),
                transform: Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    0.,
                )),
                ..Default::default()
            })
            .insert(Bullet { speed: 540. });
        audio.play(audio_res.bullet.clone());
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

fn bubble_create(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    material_res: Res<MaterialResource>,
    query: Query<&Transform, With<Aircraft>>,
) {
    if input.pressed(KeyCode::S) {
        let transform = query.single().unwrap();

        let i = fastrand::usize(0..material_res.bubble.len());
        let mut x = fastrand::f32();
        if fastrand::bool() {
            x = -x;
        }
        let y = fastrand::f32();
        let d = fastrand::i32(50..200) as f32;

        commands
            .spawn_bundle(SpriteBundle {
                material: material_res.bubble[i].clone(),
                sprite: Sprite::new(Bubble::SIZE),
                transform: Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    0.,
                )),
                ..Default::default()
            })
            .insert(Bubble {
                speed: d * Vec3::new(x, y, 0.0).normalize(),
            });
    }
}

fn bubble_move(time: Res<Time>, mut query: Query<(&mut Bubble, &mut Transform)>) {
    for (mut bubble, mut transform) in query.iter_mut() {
        transform.translation += bubble.speed * time.delta_seconds();

        if transform.translation.x > WINDOW_WIDTH / 2. - Bubble::HALF
            || transform.translation.x < -WINDOW_WIDTH / 2. + Bubble::HALF
        {
            bubble.speed.x = -bubble.speed.x;
        }

        if transform.translation.y > WINDOW_HEIGHT / 2. - Bubble::HALF
            || transform.translation.y < -WINDOW_HEIGHT / 2. + Bubble::HALF
        {
            bubble.speed.y = -bubble.speed.y;
        }
    }
}
