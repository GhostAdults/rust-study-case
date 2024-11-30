use bevy::{
    color::palettes::tailwind::{PURPLE_200, PURPLE_400},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

struct OverlayColor;
impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextStyle {
                font_size: 25.0,
                font: default(),
                color: OverlayColor::GREEN,
                ..default()
            },
        },
        ..default()
    });
}

pub fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = PURPLE_400.into();
    }
}
