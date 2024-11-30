use bevy::prelude::*;

use super::Screen;
use crate::theme::prelude::*;
use ui_palette::TITLE_BACKGROUND_COLOR;

#[derive(Component)]
struct TitleScreenRoot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
    app.add_systems(OnExit(Screen::Title), cleanup_title_screen);
}
fn spawn_title_screen(mut commonds: Commands, font: Res<UiFont>) {
    commonds
        .ui_root()
        .insert(StateScoped(Screen::Title)) // 绑定到 Screen::Title 状态
        .insert(TitleScreenRoot)
        .insert((BackgroundColor(TITLE_BACKGROUND_COLOR),))
        .with_children(|chiledren| {
            chiledren
                .button("开始游戏", &font)
                .observe(enter_gameplay_screen);
            chiledren.button("关于", &font);
            chiledren.button("退出", &font);
        });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Loading);
}

fn cleanup_title_screen(mut commands: Commands, q_title: Query<Entity, With<TitleScreenRoot>>) {
    for entity in &q_title {
        commands.entity(entity).despawn_recursive();
    }
}
