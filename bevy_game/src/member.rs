use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, add_people);
    app.add_systems(Update, (update_people, greet_people).chain());
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("Allie"))));
    commands.spawn((Person, Name(String::from("Make"))));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0)
    }
}
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Allie" {
            name.0 = "Allie.Make1".to_string();
            break;
        }
    }
}
