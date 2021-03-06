use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub struct BackgroundPlugin;
struct Background;

impl Background {
    const SPEED: f32 = 40.;
    const WIDTH: f32 = WINDOW_WIDTH;
    const HEIGHT: f32 = WINDOW_WIDTH;
}

const BOTTOM: f32 = -((WINDOW_HEIGHT - Background::HEIGHT) / 2.);

impl Plugin for BackgroundPlugin {
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
    let background = asset_server.load("images/background.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for i in 0..3 {
        let y = i as f32 * Background::WIDTH + BOTTOM;
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(background.clone().into()),
                transform: Transform::from_translation(Vec3::new(0., y, 0.)),
                sprite: Sprite::new(Vec2::new(Background::WIDTH, Background::HEIGHT)),
                ..Default::default()
            })
            .insert(Background);
    }
}

fn update(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * Background::SPEED;
        if transform.translation.y < BOTTOM - Background::HEIGHT {
            let d = BOTTOM - Background::HEIGHT - transform.translation.y;
            transform.translation.y = BOTTOM + Background::HEIGHT * 2. - d;
        }
    }
}
