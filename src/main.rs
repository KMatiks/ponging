use bevy::{prelude::*, sprite::*, window::*};


/* Entity
 - Paddle
*/

//Paddle component - identify entity that has this component is a paddle
#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let shape = Mesh2dHandle(meshes.add(Rectangle::new(10., 75.)));

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape.clone(),
        material: materials.add(Color::rgb(255., 255., 255.)),
        transform: Transform::from_xyz(
            -419.0,
            0.0,
            0.0,
        ),
        ..default()
    })
    .insert(Paddle)
    .insert(Player1);

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(Color::rgb(255., 255., 255.)),
        transform: Transform::from_xyz(
            419.0,
            0.0,
            0.0,
        ),
        ..default()
    })
    .insert(Paddle)
    .insert(Player2);
}


fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle { radius: 5.0 }));

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape.clone(),
        material: materials.add(Color::rgb(255., 255., 255.)),
        transform: Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ),
        ..default()
    })
    .insert(Ball);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(858., 525.).with_scale_factor_override(1.0),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_paddles, spawn_ball))
        .run();
}
