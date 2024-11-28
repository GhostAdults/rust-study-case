use bevy::prelude::*;

use super::Screen;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App){
      app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}
fn spawn_title_screen(mut commonds: Commands) {
    commonds.ui_root().insert(
        StateScoped(Screen::Title)
    )
    .with_children(|chiledren|
    {
        chiledren.button("开始游戏").observe(enter_gameplay_screen);
        chiledren.button("关于");
        chiledren.button("退出游戏");
    });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>){
    println!("starting game..");
}