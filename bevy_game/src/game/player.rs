use bevy::transform::commands;
use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};
use bevy_rapier2d::prelude::*;

use crate::game::config::PLAYER_RUN_SPEED;
use crate::game::level::{create_texture_atlas, PlayerBundle};
use crate::screens::Screen;

use super::config::{PLAYER_JUMP_SPEED, TILE_SIZE,PLAYER_GRAVITY_SCALE};
use super::{level::Player, player_state::PlayerState};

// 角色是否在地面上
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerGrounded(pub bool);

#[derive(Debug, Component, Default, Clone, PartialEq, Eq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());
    // 加入角色移动系统
    app.add_systems(Update, (player_run,player_jump).run_if(in_state(Screen::Gameplay)));
}

pub fn spawn_player(
    commands: &mut Commands,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    loaded_folder: &LoadedFolder,
    mut textures: &mut ResMut<Assets<Image>>,
    asset_server: &Res<AssetServer>,
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
                ..default()
            },
            facing: Facing::Right,
            collider: Collider::ball(TILE_SIZE / 2.),
            velocity: Velocity::zero(),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(0.),
            rotitatin_contrain: LockedAxes::ROTATION_LOCKED, // 锁定碰撞体为ball形
            gravity_scale: GravityScale(PLAYER_GRAVITY_SCALE),
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

// PLAYER Jump
pub fn player_jump(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Velocity,&Transform), With<Player>>,
    player_state: Res<PlayerState>,
) {
    if q_player.is_empty() {
        return;
    }
    if *player_state == PlayerState::Standing
    || *player_state == PlayerState::Runing {
     
    let (mut velocity, transform) = q_player.single_mut();
    if keyboard_input.just_pressed(KeyCode::KeyK) {
        velocity.linvel.y += PLAYER_JUMP_SPEED;
    }
       
}
}
