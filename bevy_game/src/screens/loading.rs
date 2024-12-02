//loading tex
use bevy::{asset::LoadedFolder, prelude::*};

use crate::screens::Screen;
use crate::theme::prelude::*;
use ui_palette::LOADING_BACKGROUND_COLOR;

#[derive(Resource)]
struct DelayedTimer(Timer);

#[derive(Resource, Default)]
pub struct SpriteFolder(pub Handle<LoadedFolder>);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(DelayedTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    app.add_systems(
        OnEnter(Screen::Loading),
        (spawn_loading_screen,),
    );
    app.add_systems(Update, (load_textures,check_textures).chain().run_if(in_state(Screen::Loading)));
    app.add_systems(
        Update,
        continue_to_game_screen.run_if(in_state(Screen::Loading)),
    );
    app.add_systems(OnExit(Screen::Loading), clear_loading_screen);
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    commands.insert_resource(SpriteFolder(asset_server.load_folder("textures")));
}

fn check_textures(
    mut next_state: ResMut<NextState<Screen>>,
    sprite_folder: Res<SpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprite_folder.0) {
            match sprite_folder.0.path() {
                Some(tex) => {
                    println!("加载图片资源{:#?}", tex);
                    next_state.set(Screen::Gameplay);
                }
                None => {}
            }
        }
    }
}

// or just waiting 2 secs
fn continue_to_game_screen(
    time: Res<Time>,
    mut timer: ResMut<DelayedTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_screen.set(Screen::Gameplay);
    }
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert((
            Name::new("Loading screen"),
            BackgroundColor(LOADING_BACKGROUND_COLOR),
            StateScoped(Screen::Loading),
        ))
        .with_children(|children| {
            children.label("Loading...").insert(Style {
                justify_content: JustifyContent::Center,
                ..default()
            });
        });
}

fn clear_loading_screen(mut commands: Commands, q_loading_screen: Query<(Entity, &Name)>) {
    for (entity, name) in &q_loading_screen {
        if name.as_str() == "Loading screen" {
            commands.entity(entity).despawn_recursive();
        }
    }
}
