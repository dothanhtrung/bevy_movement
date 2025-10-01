use bevy::prelude::*;
use bevy_movement::physic::{PhysicDestination, PhysicMovement};
use bevy_movement::{Arrived, MovementPluginAnyState};
use bevy_rapier3d::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let default_mat = materials.add(StandardMaterial::default());

    commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 0.4))),
            MeshMaterial3d(default_mat),
            RigidBody::Dynamic,
            Collider::cuboid(0.2, 0.2, 0.2),
            GravityScale(0.),
            PhysicMovement {
                max_velocity: 10.,
                min_velocity: 5.,
                epsilon: 0.1,
                des: vec![PhysicDestination::from_pos(Vec3::new(4., 4., 4.))],
                ..default()
            },
        ))
        .observe(arrived);

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}

#[derive(Component)]
struct Target;

fn arrived(
    trigger: On<Arrived>,
    mut query: Query<&mut PhysicMovement>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    target: Query<Entity, With<Target>>,
) {
    for entity in target.iter() {
        commands.entity(entity).despawn();
    }

    if let Ok(mut movement) = query.get_mut(trigger.target()) {
        let next_target = Vec3::new(
            fastrand::i32(-50..50) as f32 / 10.,
            fastrand::i32(-50..50) as f32 / 10.,
            fastrand::i32(-50..50) as f32 / 10.,
        );
        movement.des.push(PhysicDestination::from_pos(next_target));

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.2))),
            Transform::from_translation(next_target),
            MeshMaterial3d(materials.add(StandardMaterial::default())),
            Target,
        ));
    }
}
