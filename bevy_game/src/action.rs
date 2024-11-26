use bevy::{color::palettes::tailwind::PURPLE_200, prelude::*};

pub fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = PURPLE_200.into();
    }
}
