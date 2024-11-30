use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

use crate::game::level::{create_texture_atlas, PlayerBundle};

use super::level::Player;

pub(super) fn plugin(app: &mut App) {
    //
}

pub fn spawn_player(
    commands: &mut Commands,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    loaded_folder: &LoadedFolder,
    mut textures: &mut ResMut<Assets<Image>>,
    asset_server: &Res<AssetServer>,
    player_pos: Vec3,
) {
    let (texture_atlas_nearest, linear_texture) = create_texture_atlas(
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
                    scale: Vec3::splat(2.),
                    ..default()
                },
                ..default()
            },
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 2usize,
        },
    ));
}
