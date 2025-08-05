use crate::components::{TestMover, Velocity};
use crate::js_fn;
use crate::utils::script::JsRuntime;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

pub fn bounce_sprite(
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
