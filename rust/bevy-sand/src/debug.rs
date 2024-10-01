use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ui::node_bundles::TextBundle};

use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;

use crate::Particle;

use super::DebugText;

pub(crate) fn debug_setup(commands: &mut Commands) {
    commands.spawn((
        // TODO likely a better way to insert new lines
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 18.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 16.0,
                ..default()
            }),
            TextSection::new(
                "\nParticles: ",
                TextStyle {
                    font_size: 18.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 16.0,
                ..default()
            }),
            TextSection::new(
                "\nMouse: ",
                TextStyle {
                    font_size: 18.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 16.0,
                ..default()
            }),
        ]),
        DebugText,
    ));
}

pub(crate) fn debug_handler(
    diagnostics: Res<DiagnosticsStore>,
    windows: Query<&Window>,
    mut query: Query<&mut Text, With<DebugText>>,
    particles: Query<&Particle>,
) {
    let window = windows.single();

    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
                text.sections[3].value = particles.iter().count().to_string();
                if let Some(cursor_position) = window.cursor_position() {
                    let x = cursor_position.x;
                    let y = cursor_position.y;
                    text.sections[5].value = format!("({x:.2},{y:.2})");
                }
            }
        }
    }
}
