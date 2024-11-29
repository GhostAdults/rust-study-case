use bevy::prelude::*;
use bevy::text::Font;
use bevy_game::AppPlugin;
use bevy_ecs_ldtk::prelude::*;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(AppPlugin)    
    .run()
}
