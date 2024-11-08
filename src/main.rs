use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hw).run();
}

fn hw() {
    println!("ey up");
}