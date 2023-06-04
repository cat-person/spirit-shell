use bevy::{
    prelude::*,
    sprite::Mesh2dHandle, window::PrimaryWindow,
};

use crate::water_bundle::{WaterBall, Velocity2};

#[derive(Bundle)]
pub struct ShipBundle {
    pub ship: Ship,
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub velocity: Velocity2,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for ShipBundle {
    fn default() -> Self {
        Self {
            ship: Ship::default(),
            mesh: Mesh2dHandle::default(),
            material: Handle::<ColorMaterial>::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            velocity: Velocity2::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}

#[derive(Component)]
pub struct Ship {
    mass: f32,
}

impl Default for Ship {
    fn default() -> Self {
        Ship { mass: 100.0 }
    }
}

pub fn setup_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(ShipBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(100., 200.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        transform: Transform::from_translation(Vec3::new(0.0, -150.0, 0.0)),
        ..default()
    });
}

pub fn ship_look_at_mouse(
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut ship_query: Query<(&Ship, &mut Transform)>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.pressed(MouseButton::Right) {
        if let Ok((_, mut transform)) = ship_query.get_single_mut() {
            if let Ok(window) = window_query.get_single() {
                let (camera, camera_transform) = camera_query.single();
                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    let direction_2d = Vec2::new(
                        world_position.x - transform.translation.x,
                        world_position.y - transform.translation.y,
                    );

                    transform.rotation = if direction_2d.x.abs() < 0.00001 {
                        if (direction_2d.y < 0.0) {
                            Quat::from_xyzw(0.0, 0.0, 1.0, 0.0)
                        } else {
                            Quat::from_xyzw(0.0, 0.0, 0.0, 1.0)
                        }
                    } else {
                        let normalised_direction_2d = direction_2d.normalize();

                        let zw = Vec2::new(
                            1.0,
                            normalised_direction_2d.x / (normalised_direction_2d.y - 1.0),
                        )
                        .normalize();

                        transform.rotation.slerp(
                            Quat::from_xyzw(0.0, 0.0, zw.x, zw.y),
                            time.delta_seconds() * 5.0,
                        )
                    };
                }
            }
        }
    };
}

pub fn keyboard_controls_ship(
    time: Res<Time>,
    mut ship_query: Query<(&Ship, &mut Transform)>,
    keyboard_input_query: Res<Input<KeyCode>>,
) {
    if let Ok((_, mut transform)) = ship_query.get_single_mut() {
        let forward = transform.up();
        let right = transform.right();
        let left = transform.left();

        if keyboard_input_query.pressed(KeyCode::W) {
            transform.translation += time.delta_seconds() * forward * 200.0; // * 20.0; // Vec3::new(0.0, 2.0, 0.0);
        }

        if keyboard_input_query.pressed(KeyCode::S) {
            transform.translation += time.delta_seconds() * forward * -100.0; // Vec3::new(0.0, -2.0, 0.0);
        }

        if keyboard_input_query.pressed(KeyCode::A) {
            transform.translation += time.delta_seconds() * left * 200.0; // Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input_query.pressed(KeyCode::D) {
            transform.translation += time.delta_seconds() * right * 200.0; // Vec3::new(1.0, 0.0, 0.0);
        }
    }
}

pub fn camera_moving_after_ship(
    time: Res<Time>,
    mut camera_query: Query<(&Camera, &mut Transform), Without<Ship>>,
    ship_query: Query<&Transform, With<Ship>>,
) {
    if let Ok((_, mut camera_transform)) = camera_query.get_single_mut() {
        if let Ok(ship_transform) = ship_query.get_single() {
            camera_transform.rotation = camera_transform
                .rotation
                .slerp(ship_transform.rotation, time.delta_seconds());
            camera_transform.translation = camera_transform.translation
                + time.delta_seconds()
                    * 5.0
                    * ((ship_transform.translation - ship_transform.down() * 150.0)
                        - camera_transform.translation);
        }
    }
}

pub fn ship_repells_water_balls(
    time: Res<Time>,
    buttons: Res<Input<MouseButton>>,
    mut water_query: Query<(&WaterBall, &mut Velocity2, &Transform)>,
    ship_query: Query<(&Ship, &Transform)>,
) {
    if let Ok((ship, ship_transform)) = ship_query.get_single() {
        // if buttons.just_pressed(MouseButton::Left) {

            let x_step = 10.0; // Hardcode (I feel bad and durty)
            let y_step = 10.0; // ;-|

            let forward = ship_transform.up();

            // println!("Iterating through ship points");

            for x_step_idx in -4..4 {
                for y_step_idx in -9..9 {
                    let ship_point = ship_transform.translation + Vec3::new(
                        forward.y * x_step * x_step_idx as f32 + forward.x * y_step * y_step_idx as f32,
                        forward.x * x_step * x_step_idx as f32 + forward.y * y_step * y_step_idx as f32,
                        0.0
                    );

                    // println!("Iterating through water points l == {}", water_query.iter_mut().len() );

                    for (_, mut water_velocity, water_transform) in water_query.iter_mut() {
                        let distance = water_transform.translation.distance_squared(ship_point);

                        // println!("water_transform.translation = {} | ship_point = {} | distance = {}", water_transform.translation, ship_point, distance);

                        if distance < 1600.0 {
                            let acceleration = 100000.0 * (water_transform.translation - ship_point) / distance;
                            let delta = time.delta_seconds();

                            // println!("applied_acceleration = {}", acceleration);

                            water_velocity.x = water_velocity.x + delta * acceleration.x;
                            water_velocity.y = water_velocity.y + delta * acceleration.y;
                        }
                    }
                }
            }
        // }
    }
}

// pub fn meow_system(mut velocity_query: Query<&Velocity2>) {
//     println!("meow_system");
//     for (_) in velocity_query.iter_mut() {
//         println!("water_transform.translation");
//     }
// }

// if let Ok((_, mut camera_transform)) = camera_query.get_single_mut() {
//     if let Ok(ship_transform) = ship_query.get_single() {
//         camera_transform.rotation = camera_transform.rotation.slerp(ship_transform.rotation, time.delta_seconds());
//         camera_transform.translation = camera_transform.translation
//             + time.delta_seconds() * 5.0 * ((ship_transform.translation - ship_transform.down() * 150.0) - camera_transform.translation);
//     }
// }
