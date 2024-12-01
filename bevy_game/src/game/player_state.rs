use std::default;

use bevy::{prelude::*, reflect};

pub(super) fn plugin(app: &mut App) {
    // 注册 player 状态
    app.register_type::<PlayerState>();
    app.insert_resource(PlayerState::default());
}

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Resource)]
pub enum PlayerState {
    #[default]
    Standing,
    Runing,
    Dashing,
    Jumping,
    Climbing,
}
