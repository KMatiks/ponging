use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use rand::Rng;

use crate::{math::reflect_vec2, Collider, Movement, Paddle, HEIGHT, PADDLE_RADIUS};

pub const BALL_RADIUS: f32 = 5.0;

#[derive(Component)]
pub struct Ball;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle {
        radius: BALL_RADIUS,
    }));
    let theta = rand::thread_rng().gen_range(0.0..2.0 * std::f32::consts::PI);

    let ball = commands
        .spawn((
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .insert(Ball)
        .insert(Movement {
            velocity: Vec2 {
                x: f32::cos(theta),
                y: f32::sin(theta),
            },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
            min_speed: 0.0,
            max_speed: 0.0,
            friction: 0.0,
        }).id();
    
    // Spawn a sprite as a child of the ball
    let sprite = commands.spawn(SpriteBundle {
        texture: asset_server.load("gear.png"),
        ..default()
    }).id();
    
    commands.entity(ball).push_children(&[sprite]);
}

/// Checks for collisions between ball and all other circle colliders
pub fn handle_ball_paddle_collisions(
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

/// Add timer to avoid multiple collision calculations
pub fn handle_ball_boundary_collisions(mut ball_query: Query<(&Transform, &mut Movement), With<Ball>>) {
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
