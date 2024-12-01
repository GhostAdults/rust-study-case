use bevy::prelude::*;

mod camera;
mod config;
pub mod level;
mod player;
mod player_state;

pub fn plugin(app: &mut App) {
    //TODD -demo -
    app.add_plugins(level::plugin);
    app.add_plugins(camera::plugin);
    app.add_plugins(player::plugin);
    app.add_plugins(player_state::plugin);
}
