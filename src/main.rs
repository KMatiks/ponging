use bevy::{prelude::*, window::*};


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
        .run();
}
