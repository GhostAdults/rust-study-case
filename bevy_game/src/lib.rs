mod action;
mod camera;
mod member;
mod screens;
mod theme;
mod dev_tools;

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
        // dev

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
        /// Tick timers.
        TickTimers,
        /// Record player input.
        RecordInput,
        /// Do everything else (consider splitting this into further variants).
        Update,
}
