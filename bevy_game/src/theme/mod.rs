use bevy::prelude::*;
pub mod palette;
mod widget;
mod interaction;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        palette as ui_palette,
        widget::{Containers as _ , Widget as _},
        interaction::{InteractionPalette,OnPress},
    };
}

pub fn plugin(app: &mut App) {
       
}