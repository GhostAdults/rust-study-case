use bevy::prelude::*;
use bevy_game::AppPlugin;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(AppPlugin).run()
}
