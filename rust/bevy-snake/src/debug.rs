use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ui::node_bundles::TextBundle};

use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;

use crate::{InputBuffer, Score};

#[derive(Component)]
pub struct DebugText;

const HEADER_FONT_SIZE: f32 = 32.0;
const HEADER2_FONT_SIZE: f32 = 26.0;

pub(crate) fn debug_setup(mut commands: Commands) {
    commands.spawn((
        // TODO likely a better way to insert new lines
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: HEADER_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: HEADER2_FONT_SIZE,
                ..default()
            }),
            TextSection::new(
                "\nInput Buffer: ",
                TextStyle {
                    font_size: HEADER_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: HEADER2_FONT_SIZE,
                ..default()
            }),
            TextSection::new(
                "\nScore: ",
                TextStyle {
                    font_size: HEADER_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: HEADER2_FONT_SIZE,
                ..default()
            }),
        ]),
        DebugText,
    ));
}

pub(crate) fn debug_handler(
    diagnostics: Res<DiagnosticsStore>,
    input_buffer: Res<InputBuffer>,
    score: Res<Score>,
    windows: Query<&Window>,
    mut query: Query<&mut Text, With<DebugText>>,
) {
    let window = windows.single();

    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
                text.sections[3].value = input_buffer.0.to_string();
                text.sections[5].value = score.0.to_string();
            }
        }
    }
}
