use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ui::node_bundles::TextBundle};

use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;

use crate::components::particle::Particle;

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
        ]),
        DebugText,
    ));
}

pub(crate) fn debug_handler(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DebugText>>,
    particles: Query<&Particle>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
                text.sections[3].value = particles.iter().count().to_string();
            }
        }
    }
}
