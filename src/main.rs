use bevy::prelude::*;

pub struct HelloPlugin;


#[derive(Resource)]
struct GreetTimer(Timer);

//Allows us to add a series of systems
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// System
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string()))); //Spawns an entity (a Person entity)
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don’t need to change any other names
        }
    }
}


// SYstem (acts upon a component)
fn greet_people(time: Res<Time>, mut timer:ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {

    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
