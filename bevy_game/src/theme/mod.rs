use bevy::prelude::*;
mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use  super:: {
        widget::Containers as _,
    };
}

pub fn plugin(app: &mut App) {
     
}