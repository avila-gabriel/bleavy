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

        let pos = vec![transform.translation.x, transform.translation.y];
        let velocity = vec![vel.x, vel.y];

        let (new_pos, new_vel): (Vec<f32>, Vec<f32>) = js_fn!(
            js,
            "bounce",
            [
                pos: Vec<f32> = pos,
                vel: Vec<f32> = velocity,
                size: f32     = size,
                win_w: f32    = half_w,
                win_h: f32    = half_h
            ],
            [Vec<f32>, Vec<f32>]
        );

        if new_pos.len() == 2 {
            transform.translation.x = new_pos[0];
            transform.translation.y = new_pos[1];
        }

        if new_vel.len() == 2 {
            vel.x = new_vel[0];
            vel.y = new_vel[1];
        }
    }
}
