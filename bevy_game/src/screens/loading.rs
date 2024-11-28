use bevy::prelude::*;

use crate::theme::prelude::*;
use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
     app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
     app.add_systems(Update, continue_to_title_screen.run_if(in_state(Screen::Loading)));
}
const LOADING_BACKGROUND_COLOR: Color =  Color::srgba(0.1, 0.1, 0.1,1.0);


fn spawn_loading_screen(mut commands: Commands){
    commands.ui_root().insert((
        Name::new("Loading screen"),
        BackgroundColor(LOADING_BACKGROUND_COLOR),
        StateScoped(Screen::Loading)
    )).with_children(|children| {
        children.label("Loading...").insert(Style {
            justify_content: JustifyContent::Center,
            ..default()
        });
    });
}

fn continue_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}