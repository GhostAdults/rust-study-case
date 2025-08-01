use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};
use bevy_rapier2d::prelude::*;

use crate::game::config::PLAYER_RUN_SPEED;
use crate::game::level::{create_texture_atlas, PlayerBundle};
use crate::screens::Screen;

use super::config::{
    PLAYER_GRAVITY_SCALE, PLAYER_GRAVITY_SCALE_DOWN, PLAYER_GRAVITY_SCALE_UP,
    PLAYER_JUMP_HOLD_TIME, PLAYER_JUMP_MIN_SPEED, PLAYER_JUMP_SPEED, PLAYER_MAX_FALL_SPEED,
    TILE_SIZE,
};
use super::{level::Player, player_state::PlayerState};

// 角色是否在地面上
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerGrounded(pub bool);

// 角色朝向
#[derive(Debug, Component, Default, Clone, PartialEq, Eq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

// 跳跃组件
#[derive(Debug, Component, Default, Clone)]
pub struct JumpController {
    pub is_jumping: bool,
    pub jump_hold_timer: f32,
    pub can_variable_jump: bool, // 是否可以进行变高跳跃
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());
    // 加入角色移动系统
    app.add_systems(
        Update,
        (
            player_facing_update,
            player_animate_flip,
            player_run,
            player_jump,
            player_gravity_control,
            // limit_fall_speed,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

pub fn spawn_player(
    commands: &mut Commands,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    loaded_folder: &LoadedFolder,
    mut textures: &mut ResMut<Assets<Image>>,
    _asset_server: &Res<AssetServer>,
    player_pos: Vec3,
) {
    let (_texture_atlas_nearest, linear_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()), // 纹理为最近邻采样模式
        &mut textures,
    );
    // 创建纹理图集
    let texture_atlas_nearest = TextureAtlasLayout::from_grid(UVec2::splat(8), 16, 11, None, None);
    let texture_atlas_layout = texture_atlases.add(texture_atlas_nearest);

    // draw textures with Player
    commands.spawn((
        PlayerBundle {
            player: Player,
            sprite_bundle: SpriteBundle {
                texture: linear_texture.clone(),
                transform: Transform {
                    translation: player_pos,
                    scale: Vec3::splat(1.),
                    ..default()
                },
                sprite: Sprite {
                    flip_x: true,  // 水平翻转
                    flip_y: false, // 垂直方向不变
                    ..default()
                },
                ..default()
            },
            facing: Facing::Right,
            collider: Collider::ball(TILE_SIZE / 2.),
            velocity: Velocity::zero(),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(0.),
            rotitatin_contrain: LockedAxes::ROTATION_LOCKED, // 锁定碰撞体为ball形
            gravity_scale: GravityScale(PLAYER_GRAVITY_SCALE),
            jump_controller: JumpController::default(),
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 16 * 10 + 1usize,
        },
    ));
}

// PLAYER RUN
pub fn player_run(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_palyer: Query<&mut Velocity, With<Player>>,
    player_state: Res<PlayerState>,
) {
    if q_palyer.is_empty() {
        return;
    }
    if *player_state == PlayerState::Runing || *player_state == PlayerState::Standing {
        let mut velocity = q_palyer.single_mut();
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.linvel.x = -PLAYER_RUN_SPEED;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.linvel.x = PLAYER_RUN_SPEED;
        } else {
            velocity.linvel.x = 0.0;
        }
    }
}

// player 地面奔跑动画
pub fn player_run_animation(
    mut q_player: Query<(&mut Sprite, &Facing), With<Player>>,
    player_state: Res<PlayerState>,
) {
    if q_player.is_empty() {
        return;
    }
}

// Player stand animation switch
pub fn player_animate_flip(
    mut q_player: Query<(&mut Sprite, &Facing), With<Player>>,
    player_state: Res<PlayerState>,
) {
    if q_player.is_empty() {
        return;
    }
    if *player_state == PlayerState::Standing {
        for (mut sprite, facing) in q_player.iter_mut() {
            sprite.flip_x = match facing {
                Facing::Left => {
                    true // 水平翻转
                }
                Facing::Right => false,
            };
        }
    }
}

// PLAYER Jump - 改进的跳跃系统
pub fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Velocity, &mut JumpController), With<Player>>,
    player_state: Res<PlayerState>,
    time: Res<Time>,
) {
    if q_player.is_empty() {
        return;
    }

    let (mut velocity, mut jump_controller) = q_player.single_mut();

    // 开始跳跃
    if (*player_state == PlayerState::Standing
        || *player_state == PlayerState::Runing
        || *player_state == PlayerState::Climbing) {
        if keyboard_input.just_pressed(KeyCode::KeyK) {
            // 检查是否在地面（通过y速度判断）
            if velocity.linvel.y.abs() < 10.0 {
                velocity.linvel.y = PLAYER_JUMP_SPEED;
                jump_controller.is_jumping = true;
                jump_controller.jump_hold_timer = 0.0;
                jump_controller.can_variable_jump = true;
            }
        }
    }

    // 变高跳跃：按住跳跃键时减少重力
    if keyboard_input.pressed(KeyCode::KeyK)
        && jump_controller.is_jumping
        && jump_controller.can_variable_jump
        && velocity.linvel.y > 0.0
    {
        jump_controller.jump_hold_timer += time.delta_seconds();

        // 如果按键时间超过最大时间或达到最小速度，停止变高跳跃
        if jump_controller.jump_hold_timer >= PLAYER_JUMP_HOLD_TIME
            || velocity.linvel.y <= PLAYER_JUMP_MIN_SPEED
        {
            jump_controller.can_variable_jump = false;
        }
    }

    // 松开跳跃键时，立即停止变高跳跃
    if keyboard_input.just_released(KeyCode::KeyK) {
        jump_controller.can_variable_jump = false;
    }

    // 检查是否着陆（重置跳跃状态）
    // 简单的着陆检测：当玩家下降速度很小时认为已着陆
    if jump_controller.is_jumping && velocity.linvel.y <= 10.0 && velocity.linvel.y >= -10.0 {
        jump_controller.is_jumping = false;
        jump_controller.can_variable_jump = false;
        jump_controller.jump_hold_timer = 0.0;
    }
}

// 重力控制系统 - 实现跳跃曲线
pub fn player_gravity_control(
    mut q_player: Query<(&Velocity, &JumpController, &mut GravityScale), With<Player>>,
) {
    if q_player.is_empty() {
        return;
    }

    let (velocity, jump_controller, mut gravity_scale) = q_player.single_mut();

    if jump_controller.is_jumping {
        if velocity.linvel.y > 0.0 {
            // 上升阶段
            if jump_controller.can_variable_jump {
                // 按住跳跃键时使用较小的重力（跳得更高）
                gravity_scale.0 = PLAYER_GRAVITY_SCALE_UP;
            } else {
                // 松开跳跃键后使用正常重力
                gravity_scale.0 = PLAYER_GRAVITY_SCALE;
            }
        } else {
            // 下降阶段使用较大的重力（下降更快，更有冲击感）
            gravity_scale.0 = PLAYER_GRAVITY_SCALE_DOWN;
        }
    } else {
        // 不在跳跃状态时使用正常重力
        gravity_scale.0 = PLAYER_GRAVITY_SCALE;
    }
}

// 限制下落速度系统
fn limit_fall_speed(mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        if velocity.linvel.y < -PLAYER_MAX_FALL_SPEED {
            velocity.linvel.y = -PLAYER_MAX_FALL_SPEED;
        }
    }
}
pub fn player_facing_update(mut q_player: Query<(&Velocity, &mut Facing), With<Player>>) {
    if q_player.is_empty() {
        return;
    }
    let (velocity, mut facing) = q_player.single_mut();
    if velocity.linvel.x > 0. {
        *facing = Facing::Right;
    } else if velocity.linvel.x < 0. {
        *facing = Facing::Left;
    }
}
