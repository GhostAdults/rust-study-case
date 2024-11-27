
mod splash;

use bevy::prelude::*;

pub fn plugin(app: &mut App){
    app.init_state::<Screen>();
    app.add_plugins(splash::plugin);
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