use bevy::{ecs::query, prelude::*};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub fn plugin(app: &mut App) {
    app.insert_resource(GreetTimer(Timer::from_seconds(5.0,TimerMode::Once)));
    app.add_systems(Startup, add_people);
    app.add_systems(Update, (update_people, greet_people).chain());
}


//
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("demo"))));
    commands.spawn((Person, Name(String::from("主角"))));
    commands.spawn((Person, Name(String::from("NPC"))));
}

fn greet_people(time:Res<Time>, mut timer:ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished(){
        for name in &query {
            println!("inter game {}!", name.0)
        }
    }
}
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "demo" {
            name.0 = "sotk_demo_game".to_string();
            break;
        }
    }
}
