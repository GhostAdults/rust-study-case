use std::default;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::level::Player;

pub(super) fn plugin(app: &mut App) {
    // 注册 player 状态机
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
pub fn player_state_machine(
    mut player_state: ResMut<PlayerState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<&Velocity,With<Player>>,
){
    
}
