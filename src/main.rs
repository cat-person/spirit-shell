mod water_bundle;
mod add_water;
mod ship;

use bevy::{prelude::*, window::PresentMode};
use ship::*;
use water_bundle::*;
use add_water::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.75, 0.85, 0.90)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                canvas: Some(String::from("#main")),
                focused: true,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_water_balls)
        .add_startup_system(setup_ship)
        .add_system(water_movement)
        .add_system(update_velocity)
        .add_system(ship_repells_water_balls)
        .add_system(ship_look_at_mouse)
        .add_system(keyboard_controls_ship)
        .add_system(camera_moving_after_ship)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}



// fn movement