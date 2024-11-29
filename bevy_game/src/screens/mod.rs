
mod splash;
mod loading;
mod title;
mod gameplay;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App){
    app.init_state::<Screen>();
    app.add_plugins(splash::plugin);
    app.add_plugins(loading::plugin);
    app.add_plugins(title::plugin);
    app.add_plugins(gameplay::plugin);
}


#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    StartMenu,
    Loading,
    Title,
    Credits,
    Gameplay,
}