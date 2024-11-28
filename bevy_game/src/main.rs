use bevy::prelude::*;
use bevy_game::AppPlugin;
use bevy::text::Font;


fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(AppPlugin).run()

}
