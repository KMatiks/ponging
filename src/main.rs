pub mod ball;
pub mod math;
pub mod paddle;
pub mod common;

use ball::*;
use paddle::*;
use common::*;

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::*,
};

//rotate gear and add sprite
// move up rotate clockwise and vice versa

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WIDTH, HEIGHT)
                            .with_scale_factor_override(1.0),
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
                    ..default() //------------------------------------------------------------------
                }),
        )
        .add_systems(Startup, (spawn_camera, spawn_paddles, spawn_ball))
        .add_systems(
            Update,
            (
                apply_velocity,
                handle_keyboard_input,
                handle_gamepad_input,
                handle_ball_paddle_collisions,
                handle_ball_boundary_collisions,
            ),
        )
        .run();
}
