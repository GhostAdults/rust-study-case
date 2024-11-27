use bevy::{color::palettes::tailwind::{PURPLE_200, PURPLE_400}, prelude::*};

pub fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        println!("press spece{:?}",clear_color.0);
        clear_color.0 = PURPLE_400.into();
    }
}
