use std::default;

use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};
use bevy_ecs_ldtk::prelude::*;

use crate::screens::prelude::SpriteFolder;

use super::player::spawn_player;
pub const LEVEL_TRANSLATION_OFFSET: Vec3 = Vec3::new(-250.0, -220.0, 0.0);

// player
#[derive(Component, Default, Debug, Clone)]
pub struct Player;

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteBundle,
}

pub(super) fn plugin(app: &mut App) {
    // ldtk map
    app.insert_resource(LevelSelection::index(0));
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
        let mut translation = transform.translation + LEVEL_TRANSLATION_OFFSET;
        translation.z = 10.0;

        if entity_instance.identifier == "Player" && q_player.is_empty() {
            spawn_player(
                &mut conmmands,
                &mut texture_atlases,
                &loaded_folder,
                &mut textures,
                &asset_server,
                (transform.translation + LEVEL_TRANSLATION_OFFSET),
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
