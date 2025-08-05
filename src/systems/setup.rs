use crate::components::{TestMover, Velocity};
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn spawn_sprite(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::from(Srgba::new(1.0, 0.0, 0.0, 1.0)),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::default(),
        Velocity { x: 120.0, y: 45.0 },
        TestMover,
    ));
}
