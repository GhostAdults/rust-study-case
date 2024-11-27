use bevy::{prelude::*, render::texture::{ImageLoaderSettings, ImageSampler}, ui::UiImageBindGroups};
use crate::theme::prelude::*;

use super::Screen;

const SPLASH_BACKGROUND_COLOR: Color =  Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen);
}

fn spawn_splash_screen(mut commands: Commands,asset_server: Res<AssetServer>) {
    commands.ui_root().insert((
        Name::new("Splash screen"),
        BackgroundColor(SPLASH_BACKGROUND_COLOR),
        StateScoped(Screen::Splash)
    ))
    .with_children(|children| {
        children.spawn((
            Name::new("Splash image"),
            ImageBundle {
                style: Style{
                    margin:UiRect::all(Val::Auto),
                    width: Val::Percent(70.0),
                    ..Default::default()
                },
                image: UiImage::new(asset_server.load_with_settings(
                    "images/splash.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::linear();
                    },
                )),
                ..Default::default() 
            },
            UiImageFadeInOut {
                total_duration: SPLASH_DURATION_SECS,
                fade_duration: SPLASH_FADE_DURATION_SECS,
                t:0.0,
            }
        ));
    });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct UiImageFadeInOut {
      total_duration: f32,
      fade_duration: f32,
      t:f32,
}

impl UiImageFadeInOut {
     fn alpha(&self) -> f32 {
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;
        
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
     }
}