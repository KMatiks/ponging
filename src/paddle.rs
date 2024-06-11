use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{Collider, Movement, WIDTH};

pub const PADDLE_RADIUS: f32 = 15.0;

#[derive(Component)]
pub struct Paddle;
#[derive(Component)]
pub struct Player(pub u8);

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle {
        radius: PADDLE_RADIUS,
    }));
    let x_pos = WIDTH / 2.0 - PADDLE_RADIUS;

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: shape.clone(),
            material: materials.add(Color::rgb(255., 255., 255.)),
            transform: Transform::from_xyz(-x_pos, 0.0, 0.0),
            ..default()
        })
        .insert(Paddle)
        .insert(Movement {
            velocity: Vec2 { x: 0.0, y: 0.0 },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
            min_speed: 0.0,
            max_speed: 0.0,
            friction: 0.0,
        })
        .insert(Collider)
        .insert(Player(1));

    //Spawn Paddle entity with the following components
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(Color::rgb(255., 255., 255.)),
            transform: Transform::from_xyz(x_pos, 0.0, 0.0),
            ..default()
        })
        .insert(Paddle)
        .insert(Movement {
            velocity: Vec2 { x: 0.0, y: 0.0 },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
            min_speed: 0.0,
            max_speed: 0.0,
            friction: 0.0,
        })
        .insert(Collider)
        .insert(Player(2));
}

pub fn handle_gamepad_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    mut movement_query: Query<(&Player, &mut Movement)>,
) {
    let mut movement = movement_query
        .iter_mut()
        .find(|(player, _)| player.0 == 1)
        .expect("Player 1 not found!")
        .1;

    for gamepad in gamepads.iter() {
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp)) {
            movement.velocity = Vec2 { x: 0., y: 1. };
        } else if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown)) {
            movement.velocity = Vec2 { x: 0., y: -1. };
        } else {
            movement.velocity = Vec2 { x: 0., y: 0. };
        }
    }
}

pub fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut movement_query: Query<(&Player, &mut Movement)>,
) {
    let mut movement = movement_query
        .iter_mut()
        .find(|(player, _)| player.0 == 2)
        .expect("Player 2 not found!")
        .1;

    if keys.pressed(KeyCode::KeyW) {
        info!("pressed!");
        movement.velocity = Vec2 { x: 0., y: 1. };
    } else if keys.pressed(KeyCode::KeyS) {
        movement.velocity = Vec2 { x: 0., y: -1. };
    } else {
        movement.velocity = Vec2 { x: 0., y: 0. };
    }
}
