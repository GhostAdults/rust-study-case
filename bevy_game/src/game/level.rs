

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::screens::prelude::SpriteFolder;

use super::{
    config::TILE_SIZE,
    player::{spawn_player, Facing, JumpController},
};
pub const LEVEL_TRANSLATION_OFFSET: Vec3 = Vec3::new(-250.0, -220.0, 0.0);

//碰撞
#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub active_events: ActiveEvents,
}

// 地形
#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Terrain;

#[derive(Clone, Debug, Bundle, Default, LdtkIntCell)]
pub struct TerrainBundle {
    pub terrain: Terrain,
    #[from_int_grid_cell]
    pub collider_bundle: ColliderBundle,
}

// player
#[derive(Component, Default, Debug, Clone)]
pub struct Player;

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteBundle,
    pub facing: Facing,
    pub collider: Collider,             // 碰撞体 接触
    pub velocity: Velocity,             // 加速度
    pub rigid_body: RigidBody,          // 刚体
    pub restitution: Restitution,       // 碰撞体
    pub rotitatin_contrain: LockedAxes, // 锁定旋转
    pub gravity_scale: GravityScale,    // 重力
    pub jump_controller: JumpController, // 跳跃控制器
}

pub(super) fn plugin(app: &mut App) {
    // ldtk map
    app.insert_resource(LevelSelection::index(0));
    // 注册瓦片 
    app.register_ldtk_int_cell::<TerrainBundle>(1);
}

pub fn setup_ldtk_world(mut conmmands: Commands, asset_server: Res<AssetServer>) {
    println!("加载地图");
    conmmands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        transform: Transform::from_translation(Vec3::ZERO + LEVEL_TRANSLATION_OFFSET),
        ..Default::default()
    });
}

pub fn spawn_ldtk_entity(
    mut conmmands: Commands,
    sprite_handles: Res<SpriteFolder>,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    q_player: Query<(), With<Player>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let loaded_folder = loaded_folders.get(&sprite_handles.0).unwrap();

    // 加载贴图
    // let texture_handle = asset_server.load("textures/atlas.png");

    for (_entity, transform, entity_instance) in entity_query.iter() {
        // let texture_atlas_handle: Handle<TextureAtlasLayout> = texture_atlases.add(texture_atlas);
        // let atlas_linear_handle = texture_atlases.add(texture_atlas_linear);
        // let mut translation = transform.translation + LEVEL_TRANSLATION_OFFSET;
        // translation.z = 10.0;

        if entity_instance.identifier == "Player" && q_player.is_empty() {
            spawn_player(
                &mut conmmands,
                &mut texture_atlases,
                &loaded_folder,
                &mut textures,
                &asset_server,
                transform.translation + LEVEL_TRANSLATION_OFFSET,
            );
        }
    }
}

pub fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    texture_atlas_builder.padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!("{:?}did not an image asset", handle.path().unwrap());
            continue;
        };
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.build().unwrap();
    let texture = textures.add(texture);

    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture)
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> Self {
        if int_grid_cell.value == 1 {
            println!("creat grid collider");
            Self {
                collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                rigid_body: RigidBody::Fixed,
                restitution: Restitution::new(0.0),
                active_events: ActiveEvents::COLLISION_EVENTS,
            }
        } else {
            panic!("unsupported int grid cell value")
        }
    }
}
