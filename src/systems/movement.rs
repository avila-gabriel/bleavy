use crate::components::{TestMover, Velocity};
use crate::js_fn;
use crate::utils::script::JsRuntime;
use bevy::prelude::*;

pub fn move_sprite(
    time: Res<Time>,
    mut js: NonSendMut<JsRuntime>,
    mut query: Query<(&mut Transform, &Velocity), With<TestMover>>,
) {
    for (mut transform, vel) in query.iter_mut() {
        let (x, y) = js_fn!(
            js,
            "movement",
            [x: f32 = transform.translation.x, y: f32 = transform.translation.y, vx: f32 = vel.x, vy: f32 = vel.y, dt: f32 = time.delta_secs()],
            [f32, f32]
        );

        transform.translation.x = x;
        transform.translation.y = y;
    }
}
