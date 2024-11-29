mod action;
mod camera;
mod dev_tools;
mod member;
mod screens;
mod theme;
mod game;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // 系统默认
        app.add_plugins(DefaultPlugins);
        app.add_systems(Startup, camera::init_camera);
        app.add_systems(Update, action::change_clear_color);
        app.add_plugins(member::plugin);
        app.add_plugins(screens::plugin);
        app.add_plugins(theme::plugin);

        // ldtk level
        app.add_plugins(LdtkPlugin);
  
        app.add_plugins(game::plugin);
                
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
