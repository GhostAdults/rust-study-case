use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub const LEVEL_TRANSLATION_OFFSET: Vec3 = Vec3::new(-250.0, -220.0, 0.0);

pub(super) fn plugin(app: &mut App) {
    // ldtk map
}

pub fn setup_ldtk_world(mut conmmands: Commands, asset_server: Res<AssetServer>) {
    println!("加载地图");
    conmmands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("../assets/levels.ldtk"),
        transform: Transform::from_translation(Vec3::ZERO + LEVEL_TRANSLATION_OFFSET),
        ..Default::default()
    });
}
