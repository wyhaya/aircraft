use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::math::const_vec2;
use bevy::prelude::*;

pub struct AircraftPlugin;
pub struct Aircraft {
    speed: f32,
}

impl Aircraft {
    const WIDTH: f32 = 67.2;
    const HEIGHT: f32 = 80.0;
    pub const SIZE: Vec2 = const_vec2!([Self::WIDTH, Self::HEIGHT]);
}

impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let aircraft = asset_server.load("images/aircraft.png");
    let aircraft_y = -(WINDOW_HEIGHT / 2.0 - Aircraft::HEIGHT * 2.0);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(aircraft.into()),
            transform: Transform::from_translation(Vec3::new(0., aircraft_y, 0.)),
            sprite: Sprite::new(Aircraft::SIZE),
            ..Default::default()
        })
        .insert(Aircraft { speed: 500. });
}

fn update(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Aircraft, &mut Transform)>,
) {
    if let Ok((aircraft, mut transform)) = query.single_mut() {
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
