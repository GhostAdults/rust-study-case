use bevy::{
    color::palettes::{css::PURPLE, tailwind::PURPLE_100},
    ecs::{query, world::Command},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, add_people)
        .add_systems(Update, change_clear_color)
        .add_systems(Update, (update_people,greet_people).chain())
        .run();
}
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn add_people(mut commands: Commands) {
    commands.spawn((Person,Name(String::from("Allie"))));
    commands.spawn((Person,Name(String::from("Make"))));

}

fn greet_people(query: Query<&Name,With<Person>>){
    for name in &query {
        println!("hello {}!",name.0)
    }
}
fn update_people(mut query:Query<&mut Name,With<Person>>) {
    for  mut name  in &mut query {
         if name.0  == "Allie" {
            name.0 = "Allie.Make1".to_string();
            break;
         }
    }
}

fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = PURPLE_100.into();
    }
}
