
mod splash;
mod loading;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App){
    app.init_state::<Screen>();
    app.add_plugins(splash::plugin);
    app.add_plugins(loading::plugin);
    app.add_plugins(title::plugin);
}


#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Gameplay,
}