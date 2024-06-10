mod math;
use math::reflect_vec2;

use bevy::{math::bounding::*, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, sprite::*, window::*};
use rand::Rng;


const WIDTH: f32 = 858.0;
const HEIGHT: f32 = 525.0;
const BALL_RADIUS: f32 = 5.0;
const PADDLE_RADIUS: f32 = 15.0;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct CollisionArea;

#[derive(Component)]
struct CircleCollider {
    radius: f32
}

#[derive(Component)]
struct Player(u8);

#[derive(Component)]
struct Movement {
    velocity: Vec2,
    acceleration: Vec2,
    min_speed:f32,
    max_speed: f32
}

#[derive(Component)]
struct Collider;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let shape = Mesh2dHandle(meshes.add(Circle { radius: PADDLE_RADIUS }));
    let x_pos = WIDTH / 2.0 - PADDLE_RADIUS;

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape.clone(),
        material: materials.add(Color::rgb(255., 255., 255.)),
        transform: Transform::from_xyz(
            -x_pos,
            0.0,
            0.0,
        ),
        ..default()
    })
    .insert(Paddle)
    .insert(Movement {
        velocity: Vec2 { x: 0.0, y:  0.0},
        acceleration: Vec2 { x: 0.0, y:  0.0},
        min_speed: 0.0,
        max_speed: 0.0
    })
    .insert(Collider)
    .insert(Player(1));

    //Spawn Paddle entity with the following components
    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(Color::rgb(255., 255., 255.)),
        transform: Transform::from_xyz(
            x_pos,
            0.0,
            0.0,
        ),
        ..default()
    })
    .insert(Paddle)
    .insert(Movement {
        velocity: Vec2 { x: 0.0, y:  0.0},
        acceleration: Vec2 { x: 0.0, y:  0.0},
        min_speed: 0.0,
        max_speed: 0.0
    })
    .insert(Collider)
    .insert(Player(2));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle { radius: BALL_RADIUS }));
    let theta = rand::thread_rng().gen_range(0.0 .. 2.0*std::f32::consts::PI);

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
    .insert(Ball)
    .insert(Movement {
        velocity: Vec2 { x: f32::cos(theta), y:  f32::sin(theta)},
        acceleration: Vec2 { x: 0.0, y:  0.0},
        min_speed: 0.0,
        max_speed: 0.0,
    });
}

fn handle_gamepad_input(
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

fn handle_keyboard_input(
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

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
    for (mut transform, mut movement) in &mut query {
        let delta_velocity = movement.acceleration * time.elapsed_seconds();
        movement.velocity += delta_velocity;
        transform.translation += Vec3::new(movement.velocity.x, movement.velocity.y, 0.0);
    }
}

///Checks for collisions between ball and all other circle colliders
fn handle_ball_paddle_collisions(
    mut ball_query: Query<(&Transform, &mut Movement), With<Ball>>,
    paddle_query: Query<(&Transform, &Paddle), With<Collider>>,
) {
    let (ball_transform, mut ball_movement) = ball_query.single_mut();
    let ball_pos = ball_transform.translation.truncate();

    for (paddle_transform, paddle) in &paddle_query {
        let paddle_pos = paddle_transform.translation.truncate();

        let distance = (paddle_pos - ball_pos).length();

        if distance <= BALL_RADIUS + PADDLE_RADIUS {
            let collision_normal = (ball_pos - paddle_pos).normalize();
            ball_movement.velocity = reflect_vec2(ball_movement.velocity, collision_normal);
        }

    }
}


//Send event with point of collisions

// fn handle_ball_wall_collisions(mut &bal)

fn handle_ball_boundary_collisions(
    mut ball_query: Query<(&Transform, &mut Movement), With<Ball>>,
) {
    let (ball_transform, mut ball_movement) = ball_query.single_mut();
    let ball_pos = ball_transform.translation.truncate();

    if ball_pos.y + BALL_RADIUS > HEIGHT / 2.0 {
        let boundary_pos = Vec2::new(ball_pos.x, HEIGHT / 2.0);
        let collision_normal = (ball_pos - boundary_pos).normalize();
        ball_movement.velocity = reflect_vec2(ball_movement.velocity, collision_normal);
    }


    if ball_pos.y - BALL_RADIUS < -HEIGHT / 2.0 {
        let boundary_pos = Vec2::new(ball_pos.x, -HEIGHT / 2.0);
        let collision_normal = (ball_pos - boundary_pos).normalize();
        ball_movement.velocity = reflect_vec2(ball_movement.velocity, collision_normal);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WIDTH, HEIGHT).with_scale_factor_override(1.0),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            //------------------------------------------------------------------
            // Fix for windows terminal error spam due to wgpu bug
            // See this issue: https://github.com/bevyengine/bevy/issues/9975
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    #[cfg(target_os = "windows")]
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            //------------------------------------------------------------------
            }))
        .add_systems(Startup, (spawn_camera, spawn_paddles, spawn_ball))
        .add_systems(Update, (apply_velocity, handle_keyboard_input, handle_gamepad_input, handle_ball_paddle_collisions, handle_ball_boundary_collisions))
        .run();
}
