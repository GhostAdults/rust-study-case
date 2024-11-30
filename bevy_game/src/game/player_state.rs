use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // player
}

pub enum PlayerState {
    Standing,
    Runing,
    Dashing,
    Jumping,
    Climbing,
}
