use bevy::prelude::*;
mod interaction;
pub mod palette;
mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, OnPress},
        palette as ui_palette,
        widget::{Containers as _, Widget as _},
        UiFont,
    };
}

#[derive(Resource, Debug)]
pub struct UiFont(Handle<Font>);

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_fonts);
}

fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load(palette::FONT_FAMILY_MISANS);
    commands.insert_resource(UiFont(font)); //加入为一个句柄
}
