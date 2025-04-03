use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_rapier2d::prelude::Velocity;
use rand::prelude::*;
use std::time::Duration;

use crate::{
    audio::SoundEffect, demo::level::LevelAssets, physics::ground::GroundDetection, AppSet,
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.register_type::<PlayerAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSet::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .run_if(resource_exists::<LevelAssets>)
                .in_set(AppSet::Update),
        ),
    );
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(
        &Velocity,
        &mut Sprite,
        &mut PlayerAnimation,
        &GroundDetection,
    )>,
) {
    for (velocity, mut sprite, mut animation, ground) in &mut player_query {
        let dx = velocity.linvel.x;
        if dx != 0.0 {
            sprite.flip_x = dx < 0.0;
        }

        let animation_state = if ground.on_ground {
            if velocity.linvel.x != 0. {
                PlayerAnimationState::Walking
            } else {
                PlayerAnimationState::Idling
            }
        } else {
            if velocity.linvel.y >= 0. {
                PlayerAnimationState::Jumping
            } else {
                PlayerAnimationState::Falling
            }
        };
        animation.update_state(animation_state);
    }
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the
/// animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    assets: Res<LevelAssets>,
    mut step_query: Query<&PlayerAnimation>,
) {
    for animation in &mut step_query {
        if animation.state == PlayerAnimationState::Walking
            && animation.changed()
            && (animation.frame == 2 || animation.frame == 5)
        {
            let rng = &mut rand::thread_rng();
            let random_step = assets.audio_steps.choose(rng).unwrap();
            commands.spawn((
                AudioPlayer(random_step.clone()),
                PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(0.3),
                    ..PlaybackSettings::ONCE
                },
                SoundEffect,
            ));
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self::idling()
    }
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idling,
    Walking,
    Jumping,
    Falling,
}

impl PlayerAnimation {
    /// The number of frames and their duration for each animation.
    const IDLE_FRAMES: usize = 1;
    const IDLE_INTERVAL: Duration = Duration::from_millis(100);
    const WALKING_FRAMES: usize = 4;
    const WALKING_INTERVAL: Duration = Duration::from_millis(100);
    const JUMPING_FRAMES: usize = 1;
    const JUMPING_INTERVAL: Duration = Duration::from_millis(50);
    const FALLING_FRAMES: usize = 1;
    const FALLING_INTERVAL: Duration = Duration::from_millis(50);

    fn idling() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling,
        }
    }

    fn walking() -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking,
        }
    }

    fn jumping() -> Self {
        Self {
            timer: Timer::new(Self::JUMPING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Jumping,
        }
    }

    fn falling() -> Self {
        Self {
            timer: Timer::new(Self::FALLING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Falling,
        }
    }

    pub fn new() -> Self {
        Self::idling()
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Idling => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking => Self::WALKING_FRAMES,
                PlayerAnimationState::Jumping => Self::JUMPING_FRAMES,
                PlayerAnimationState::Falling => Self::FALLING_FRAMES,
            };
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        if self.state != state {
            match state {
                PlayerAnimationState::Idling => *self = Self::idling(),
                PlayerAnimationState::Walking => *self = Self::walking(),
                PlayerAnimationState::Jumping => *self = Self::jumping(),
                PlayerAnimationState::Falling => *self = Self::falling(),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::Idling => 1 + self.frame,
            PlayerAnimationState::Walking => 6 + self.frame,
            PlayerAnimationState::Jumping => 2 + self.frame,
            PlayerAnimationState::Falling => 4 + self.frame,
        }
    }
}
