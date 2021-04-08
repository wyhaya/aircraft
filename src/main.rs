mod aircraft;
mod background;
mod bullet;
mod fps;
mod obstacle;
mod score;

use aircraft::{Aircraft, AircraftPlugin};
use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bullet::{Bullet, BulletPlugin};
use fps::FpsPlugin;
use obstacle::{Obstacle, ObstaclePlugin};
use score::{Score, ScorePlugin};

pub const WINDOW_WIDTH: f32 = 640.;
pub const WINDOW_HEIGHT: f32 = 1000.;

fn main() {
    let window = WindowDescriptor {
        title: env!("CARGO_PKG_NAME").to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        ..Default::default()
    };
    App::build()
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_plugin(FpsPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(AircraftPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(ObstaclePlugin)
        .add_startup_system(setup.system())
        .add_system(update.system())
        .run();
}

struct Sounds {
    explosion: Handle<AudioSource>,
    injured: Handle<AudioSource>,
    // TODO
    death: Handle<AudioSource>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Sounds {
        explosion: asset_server.load("sounds/explosion.mp3"),
        injured: asset_server.load("sounds/injured.mp3"),
        death: asset_server.load("sounds/death.mp3"),
    });
}

fn update(
    mut commands: Commands,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut score_query: Query<&mut Score>,
    aircraft_query: Query<&Transform, With<Aircraft>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    let mut score = score_query.single_mut().unwrap();
    let aircraft_position = aircraft_query.single().unwrap().translation;

    'obstacle: for (obstacle_entity, transform) in obstacle_query.iter() {
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
            score.collision();
            commands.entity(obstacle_entity).despawn();
            audio.play(sounds.injured.clone());
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
                score.hit();
                commands.entity(obstacle_entity).despawn();
                commands.entity(bullet_entiry).despawn();
                audio.play(sounds.explosion.clone());
                continue 'obstacle;
            }
        }
    }
}
