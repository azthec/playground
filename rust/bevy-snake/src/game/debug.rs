use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ui::node_bundles::TextBundle};

use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;

use crate::AppSet;

use super::game::Score;
use super::input::InputBuffer;

#[derive(Component)]
pub struct DebugText;

pub enum DebugType {
    FPS,
    Input,
    Score,
}

#[derive(Event)]
pub struct DebugEvent {
    kind: DebugType,
    value: String,
}

const HEADER_FONT_SIZE: f32 = 32.0;
const HEADER2_FONT_SIZE: f32 = 26.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, debug_setup);
    app.add_systems(FixedUpdate, debug_handler.in_set(AppSet::PostUpdate));
}

fn debug_setup(mut commands: Commands) {
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

fn debug_handler(
    diagnostics: Res<DiagnosticsStore>,
    input_buffer: Res<InputBuffer>,
    score: Res<Score>,
    mut query: Query<&mut Text, With<DebugText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
            if input_buffer.is_changed() {
                text.sections[3].value = input_buffer.0.to_string();
            }
            if score.is_changed() {
                text.sections[5].value = score.0.to_string();
            }
        }
    }
}
