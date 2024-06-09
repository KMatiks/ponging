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

#[derive(Component)]
struct MovementSpeed { speed: f32 }

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
    .insert(MovementSpeed { speed: 100. })
    .insert(Player1);

    //Spawn Paddle entity with the following components
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
    .insert(MovementSpeed { speed: 100. })
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

fn handle_gamepad_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut transform_query: Query<(&mut Transform, &MovementSpeed), With<Player1>>,
    time: Res<Time>
) {
    for gamepad in gamepads.iter() {
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp)) {
            info!("Pressed DPAD Up");
            let (mut translation, speed) = transform_query.single_mut();

            translation.translation.y += speed.speed * time.delta_seconds();
        } else if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown)) {
            info!("Pressed DPAD Down");
            let (mut translation, speed) = transform_query.single_mut();

            translation.translation.y -= speed.speed * time.delta_seconds();
        }

        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
    }
}

fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut transform_query: Query<(&mut Transform, &MovementSpeed), With<Player2>>,
    time: Res<Time>
) {
    let (mut translation, speed) = transform_query.single_mut(); 

    if keys.pressed(KeyCode::ArrowUp) {
        translation.translation.y += speed.speed * time.delta_seconds();
    } else if keys.pressed(KeyCode::ArrowDown) {
        translation.translation.y -= speed.speed * time.delta_seconds();
    }
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
        .add_systems(Update, (handle_gamepad_input, handle_keyboard_input))
        .run();
}
