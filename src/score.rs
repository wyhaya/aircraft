use bevy::prelude::*;

pub struct ScorePlugin;
pub struct Score(i32);

impl Score {
    pub fn hit(&mut self) {
        self.0 += 1;
    }
    pub fn collision(&mut self) {
        self.0 -= 10;
    }
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update.system());
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");
    commands.spawn_bundle(UiCameraBundle::default());
    // Score
    commands
        .spawn_bundle(TextBundle {
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
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 26.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 26.0,
                            color: Color::ORANGE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Score(0));
}

fn update(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in query.iter_mut() {
        text.sections[1].value = format!("{}", score.0);
    }
}
