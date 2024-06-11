use bevy::prelude::*;

pub const WIDTH: f32 = 858.0;
pub const HEIGHT: f32 = 525.0;

#[derive(Component)]
pub struct CollisionArea;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct Movement {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub min_speed: f32, //friction up to
    pub max_speed: f32,
    pub friction: f32,
}

#[derive(Component)]
pub struct Collider;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
    for (mut transform, mut movement) in &mut query {
        let delta_velocity = movement.acceleration * time.elapsed_seconds();
        movement.velocity += delta_velocity;
        transform.translation += Vec3::new(movement.velocity.x, movement.velocity.y, 0.0);
    }
}