use bevy::{input::mouse::MouseMotion, prelude::*, sprite::Mesh2dHandle, window::PrimaryWindow};

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

#[derive(Component)]
pub struct Velocity2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity2 {
    fn default() -> Self {
        Velocity2 { x: 1.0, y: 1.0 }
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
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ship_query: Query<(&Ship, &mut Transform)>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    if !motion_evr.is_empty() {
        if let Ok((_, mut transform)) = ship_query.get_single_mut() {
            if let Ok(window) = window_query.get_single() {
                if let Some(coursor_position) = window.cursor_position() {
                    let coursor_position_modified =
                        coursor_position - Vec2::new(window.width() / 2.0, window.height() / 2.0);
                    // transform.look_to(Vec3::new(coursor_position_modified.x, coursor_position_modified.y, 0.0), Vec3::Up);

                    let direction_2d = Vec2::new(
                        coursor_position_modified.x - transform.translation.x,
                        coursor_position_modified.y - transform.translation.y,
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
                            normalised_direction_2d.x / (normalised_direction_2d.y + 1.0),
                        ).normalize();
                        Quat::from_xyzw(0.0, 0.0, zw.x, zw.y)
                    };
                }
            }
        }
    }
}
