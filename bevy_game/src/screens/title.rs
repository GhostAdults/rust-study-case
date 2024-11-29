use bevy::prelude::*;

use super::Screen;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}
fn spawn_title_screen(mut commonds: Commands, font: Res<UiFont>) {
    commonds
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|chiledren| {
            chiledren
                .button("新游戏", &font)
                .observe(enter_gameplay_screen);
            chiledren.button("关于", &font);
            chiledren.button("退出", &font);
        });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>,mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}
