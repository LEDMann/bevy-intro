use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hw, greet_people))
        .run();
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