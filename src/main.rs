use crate::plugins::game::GamePlugin;
use crate::utils::script::JsRuntime;
use bevy::prelude::*;

mod components;
mod plugins;
mod systems;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_non_send_resource(JsRuntime::new())
        .add_plugins(GamePlugin)
        .run();
}
