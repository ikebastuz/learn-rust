use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ScoreText {
    pub score: usize,
}

pub fn setup_stats(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..default()
                },
            ),
        ]),
        FpsText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..default()
                },
            ),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        ScoreText { score: 0 },
    ));
}

pub fn update_stats(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}
