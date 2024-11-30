mod dev_tools;
mod game;
mod member;
mod overlay;
mod screens;
mod theme;

use bevy::{
    prelude::*,
    render::texture::{ImagePlugin, ImageSamplerDescriptor},
    window::{MonitorSelection, Window, WindowPlugin, WindowPosition, WindowResolution},
};
use bevy_ecs_ldtk::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // 系统默认
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor::nearest(),
                })
                .set(WindowPlugin {
                    //设置窗口大小 1100*750
                    primary_window: Some(Window {
                        #[cfg(target_os = "windows")]
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: WindowResolution::new(1200.0, 800.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        );
        app.add_systems(Update, overlay::change_clear_color);
        app.add_plugins(overlay::plugin);
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
