use bevy::prelude::*;

use crate::game::level::{
    setup_ldtk_world as spawn_level_command, spawn_ldtk_entity as spawn_ldtk_entity_c,
};
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level_command);
    app.add_systems(
        PreUpdate,
        (spawn_ldtk_entity_c).run_if(in_state(Screen::Gameplay)),
    );
}
