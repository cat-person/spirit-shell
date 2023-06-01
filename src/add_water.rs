use bevy::{prelude::*, input::mouse::{MouseMotion}, window::PrimaryWindow, ecs::query};

use crate::water_bundle::{WaterBall, WaterBallBundle, Velocity2};

pub fn mouse_button_input(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut water_query: Query<(Entity, &WaterBall, &mut Transform)>,
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    let window = window_query.get_single().unwrap();

    let coordinate_transform = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    
    

    if buttons.just_pressed(MouseButton::Left) {
        let coursor_position = window.cursor_position().unwrap() - coordinate_transform;
        // let mut mouse_movement = Vec2::new(0.0, 0.0);

        // let event_count = motion_evr.len() as f32;

        // motion_evr.iter().for_each(|event| {
        //     println!("Mouse moved: delta_X: {} px, delta_Y: {} px", event.delta.x, event.delta.y);
        //     mouse_movement += event.delta
        // });

    
        // println!("Mouse moved: X: {} px, Y: {} px", mouse_movement.x, mouse_movement.y);
        
        commands.spawn(WaterBallBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(coursor_position.x, coursor_position.y, 0.)),
            // velocity: Velocity2 {x: vx_distr.sample(&mut rng), y: vy_distr.sample(&mut rng)},
            ..default()
        });
    }
    if buttons.just_pressed(MouseButton::Right) {
        let coursor_position = window.cursor_position().unwrap() - coordinate_transform;
        water_query.into_iter().for_each(|(entity, _, transform)| {
            
            let delta_x = transform.translation.x - coursor_position.x;
            let delta_y = transform.translation.y - coursor_position.y;

            if delta_x * delta_x + delta_y * delta_y < 400.0 {
                commands.entity(entity).despawn()    
            }
        })
    }

}