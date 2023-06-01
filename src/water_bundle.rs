use std::ops::Add;

use bevy::{prelude::*, sprite::Mesh2dHandle, window::PrimaryWindow};
use rand::{distributions::Uniform, prelude::Distribution};

#[derive(Bundle)]
pub struct WaterBallBundle {
    pub water_ball: WaterBall,
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub velocity: Velocity2,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for WaterBallBundle {
    fn default() -> Self {
        Self {
            water_ball: WaterBall::default(),
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
pub struct WaterBall {}

impl Default for WaterBall {
    fn default() -> Self {
        WaterBall {}
    }
}

#[derive(Component)]
pub struct Velocity2 {
    x: f32,
    y: f32
}

impl Default for Velocity2 {
    fn default() -> Self {
        Velocity2 { x: 1.0, y: 1.0 }
    }
}

// pub fn setup_grid(mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>){
//     let window: &Window = window_query.get_single().unwrap();

//     let vertical_sectors_count = 10.0;

//     let square_side = window.height() / vertical_sectors_count;

//     for vertical_line_idx in 0..(vertical_sectors_count as i32 - 1) {
//         commands.spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::GRAY,
//                 custom_size: Some(Vec2::new(window.width(), 4.0)),
//                 ..default()
//             },
//             transform: Transform::from_translation(Vec3::new(0., (vertical_line_idx as f32 - (vertical_sectors_count / 2.0) + 1.0) * square_side, 0.)),
//             ..default()
//         });
//     }

//     let horisontal_sectors_count = (window.width() / square_side).ceil();

//     for horisontal_line_idx in 0..(horisontal_sectors_count as i32 - 1) {
//         commands.spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::DARK_GRAY,
//                 custom_size: Some(Vec2::new(4.0, window.height())),
//                 ..default()
//             },
//             transform: Transform::from_translation(Vec3::new((horisontal_line_idx as f32 - (horisontal_sectors_count / 2.0) + 1.0) * square_side, 0., 0.)),
//             ..default()
//         });
//     }
// }

pub fn setup_water_balls(mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    // let vertical_sectors_count = 10.0;

    // let square_side = window.height() / vertical_sectors_count;

    let window = window_query.get_single().unwrap();
    let x_max = window.width() / 2.0 - 20.0;
    let y_max = window.height() / 2.0 - 20.0;
    
    let mut rng = rand::thread_rng();
    let x_distr = Uniform::new(-x_max, x_max);
    let y_distr = Uniform::new(-y_max, y_max);
    
    let vx_distr = Uniform::new(-200.0, 200.0);
    let vy_distr = Uniform::new(-200.0, 200.0);

    for _ in 0..10 {
        commands.spawn(WaterBallBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x_distr.sample(&mut rng), y_distr.sample(&mut rng), 0.)),
            velocity: Velocity2 {x: vx_distr.sample(&mut rng), y: vy_distr.sample(&mut rng)},
            ..default()
        });
    }
}

pub fn water_movement (window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut water_query: Query<(Entity, &WaterBall, &mut Transform, &mut Velocity2)>,
) {
    let window = window_query.get_single().unwrap();
    let x_max = window.width() / 2.0 - 20.0;
    let y_max = window.height() / 2.0 - 20.0;

    for (_, _, mut transform, mut velocity) in water_query.iter_mut() {
        transform.translation = Vec3{ 
            x: transform.translation.x + time.delta_seconds() * velocity.x, 
            y: transform.translation.y + time.delta_seconds() * velocity.y,
            z: 0.0, 
        };

        // if(x_max < transform.translation.x && 0.0 < velocity.x) 
        //     || (transform.translation.x < -x_max && velocity.x < 0.0){
        //     velocity.x = -velocity.x
        // }

        // if(y_max < transform.translation.y && 0.0 < velocity.y) 
        //     || (transform.translation.y < -y_max && velocity.y < 0.0){
        //     velocity.y = -velocity.y
        // }
    }
}

pub fn update_velocity(
    time: Res<Time>,
    water_query: Query<(Entity, &WaterBall, &Transform)>,
    mut velocity_query: Query<(Entity, &WaterBall, &Transform, &mut Velocity2)>
) {
    for (self_entity, _, self_position, mut self_velocity) in velocity_query.iter_mut() {
        for (other_entity, _, other_position) in water_query.iter() {
            if self_entity != other_entity {
                let distance = self_position.translation.distance_squared(other_position.translation);
                let acceleration = (self_position.translation - other_position.translation) / distance;
                
                let delta = time.delta_seconds() * 1000.0;
                
                self_velocity.x = self_velocity.x + delta * acceleration.x;
                self_velocity.y = self_velocity.y + delta * acceleration.y;
            }
        }

        self_velocity.x *= 0.99;
        self_velocity.y *= 0.99;

        let center_attraction = - self_position.translation / 20.0;

        self_velocity.x += 20.0 * time.delta_seconds() * center_attraction.x;
        self_velocity.y += 20.0 * time.delta_seconds() * center_attraction.y;
    }
}