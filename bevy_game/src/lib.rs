mod action;
mod camera;
mod member;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)));
        app.add_plugins(DefaultPlugins);
        app.add_systems(Startup, camera::init_camera);
        app.add_systems(Update, action::change_clear_color);
        app.add_plugins(member::plugin);
    }
}
