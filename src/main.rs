use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hw, (update_people, greet_people).chain()));
    }
}

fn hw() {
    println!("ey up");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Lewis".to_string())));
    commands.spawn((Person, Name("Zumbu".to_string())));
    commands.spawn((Person, Name("Hume".to_string())));
} 

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Zumbu" {
            name.0 = "Ubmuz".to_string();
            break;
        }
    }
}