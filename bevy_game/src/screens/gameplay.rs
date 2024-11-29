use bevy::prelude::*;

use crate::game::level::setup_ldtk_world as spawn_level_command;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level_command);
}
