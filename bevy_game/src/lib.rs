mod action;
mod camera;
mod member;
mod screens;
mod theme;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // 系统默认
        app.add_plugins(DefaultPlugins);
        app.add_systems(Startup, camera::init_camera);
        app.add_systems(Update, action::change_clear_color);
        app.add_plugins(member::plugin);
        app.add_plugins(screens::plugin);
    }
}
