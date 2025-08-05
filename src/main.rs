use bevy::core_pipeline::core_2d::Camera2d;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

#[macro_use]
mod utils;
use crate::utils::JsRuntime;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct TestMover;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_non_send_resource(JsRuntime::new())
        .add_systems(Startup, (spawn_camera, spawn_sprite))
        .add_systems(Update, (move_sprite, bounce_sprite))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_sprite(mut commands: Commands) {
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

fn move_sprite(
    time: Res<Time>,
    mut js: NonSendMut<JsRuntime>,
    mut query: Query<(&mut Transform, &Velocity), With<TestMover>>,
) {
    for (mut transform, vel) in query.iter_mut() {
        let (x, y) = js_fn!(
            js,
            "move",
            [x: f32 = transform.translation.x, y: f32 = transform.translation.y, vx: f32 = vel.x, vy: f32 = vel.y, dt: f32 = time.delta_secs()],
            [f32, f32]
        );

        transform.translation.x = x;
        transform.translation.y = y;
    }
}

fn bounce_sprite(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut js: NonSendMut<JsRuntime>,
    mut query: Query<(&mut Transform, &mut Velocity, &Sprite), With<TestMover>>,
) {
    let Ok(win) = windows.single() else { return };
    let (half_w, half_h) = (win.width() * 0.5, win.height() * 0.5);

    for (mut transform, mut vel, sprite) in query.iter_mut() {
        let size = sprite.custom_size.unwrap_or(Vec2::ZERO).x;

        let (x, y, vx, vy) = js_fn!(
            js,
            "bounce",
            [x: f32 = transform.translation.x, y: f32 = transform.translation.y, vx: f32 = vel.x, vy: f32 = vel.y, size: f32 = size, win_w: f32 = half_w, win_h: f32 = half_h],
            [f32, f32, f32, f32]
        );

        transform.translation.x = x;
        transform.translation.y = y;
        vel.x = vx;
        vel.y = vy;
    }
}
