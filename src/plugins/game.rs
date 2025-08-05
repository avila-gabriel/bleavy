use crate::systems::setup::{spawn_camera, spawn_sprite};
use crate::systems::{bounce::bounce_sprite, movement::move_sprite};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_sprite))
            .add_systems(Update, (move_sprite, bounce_sprite.after(move_sprite)));
    }
}
