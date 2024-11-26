use bevy::{prelude::*, render::view::window};

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct SnakeHead;

#[derive(Resource)]
struct Materials {
    head_materials: Handle<ColorMaterial>,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

struct Entity(u64);

fn add_people(mut conmmands: Commands) {
    conmmands.spawn((Person, Name("Allie".to_string())));
}

fn setup(mut conmmands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    conmmands.spawn(Camera2dBundle::default());
    conmmands.insert_resource(Materials {
        head_materials: materials.add(SNAKE_HEAD_COLOR),
    });
}
fn game_setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn size_scaling(windows: Query<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            pos.y += 1;
        }
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("{}", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Startup, setup);
        app.add_systems(Startup, game_setup);
        app.add_systems(
            PostUpdate, // 替代 CoreStage::PostUpdate
            (position_translation, size_scaling),
        );
        app.add_systems(Update, snake_movement);
        app.add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(HelloPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),       // 设置窗口标题
                resolution: (500.0, 500.0).into(), // 设置窗口宽高
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .run();
}
