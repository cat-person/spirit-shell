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
    pub x: f32,
    pub y: f32
}

impl Default for Velocity2 {
    fn default() -> Self {
        Velocity2 { x: 1.0, y: 1.0 }
    }
}

pub fn setup_water_balls(mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    let window = window_query.get_single().unwrap();
    let x_max = window.width() / 2.0 - 20.0;
    let y_max = window.height() / 2.0 - 20.0;
    
    let mut rng = rand::thread_rng();
    let x_distr = Uniform::new(-x_max, x_max);
    let y_distr = Uniform::new(-y_max, y_max);

    for _ in 0..500 {
        commands.spawn(WaterBallBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x_distr.sample(&mut rng), y_distr.sample(&mut rng), 0.)),
            ..default()
        });
    }
}

pub fn water_movement (time: Res<Time>,
    mut water_query: Query<(Entity, &WaterBall, &mut Transform, &mut Velocity2)>,
) {
    for (_, _, mut transform, mut velocity) in water_query.iter_mut() {
        transform.translation = Vec3{ 
            x: transform.translation.x + time.delta_seconds() * velocity.x, 
            y: transform.translation.y + time.delta_seconds() * velocity.y,
            z: 0.0, 
        };
    }
}

pub fn update_velocity(
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    water_query: Query<(Entity, &WaterBall, &Transform)>,
    mut velocity_query: Query<(Entity, &WaterBall, &Transform, &mut Velocity2)>
) {
    let window = window_query.get_single().unwrap();
    let x_max = window.width() / 2.0 - 20.0;
    let y_max = window.height() / 2.0 - 20.0;

    for (self_entity, _, self_position, mut self_velocity) in velocity_query.iter_mut() {
        for (other_entity, _, other_position) in water_query.iter() {
            if self_entity != other_entity {
                let distance = self_position.translation.distance_squared(other_position.translation);
                let acceleration = (self_position.translation - other_position.translation) / distance;
                
                if distance < 90000.0 {
                    let delta = time.delta_seconds() * 1000.0;
                    
                    self_velocity.x = self_velocity.x + delta * acceleration.x;
                    self_velocity.y = self_velocity.y + delta * acceleration.y;
                }
            }
        }

        if self_position.translation.x < -x_max {
            self_velocity.x += time.delta_seconds() * (-x_max - self_position.translation.x);
        }

        if x_max < self_position.translation.x {
            self_velocity.x += time.delta_seconds() * (x_max - self_position.translation.x);
        }

        if self_position.translation.y < -y_max {
            self_velocity.y += time.delta_seconds() * (-y_max - self_position.translation.y);
        }

        if y_max < self_position.translation.y {
            self_velocity.y += time.delta_seconds() * (y_max - self_position.translation.y);
        }

        self_velocity.x *= 0.999;
        self_velocity.y *= 0.999;

        self_velocity.x = self_velocity.x.min(100.0);
        self_velocity.x = self_velocity.x.max(-100.0);

        self_velocity.y = self_velocity.y.min(100.0);
        self_velocity.y = self_velocity.y.max(-100.0);
    }
}