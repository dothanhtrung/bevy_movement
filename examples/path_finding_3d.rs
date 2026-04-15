//! Move object by mouse click with path finding provided by bevy_northstar.
//! You will need to set up your grid with bevy_northstar first.
//! NOT FINISHED YET.

use bevy::prelude::*;
use bevy_movement::linear::{
    GridInfo,
    LinearMovement,
};
use bevy_movement::mouse_control::{
    ClickCatcher,
    MouseMovementObject,
};
use bevy_movement::MovementPluginAnyState;
use bevy_northstar::components::AgentPos;
use bevy_northstar::plugin::NorthstarPlugin;
use bevy_northstar::prelude::{
    CardinalNeighborhood,
    GridSettingsBuilder,
    Nav,
};
use bevy_northstar::CardinalGrid;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    // NorthstarPlugin for path finding
    app.add_plugins(NorthstarPlugin::<CardinalNeighborhood>::default());

    app.add_plugins(MovementPluginAnyState::any()) // This plugin
        .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    window: Single<&Window>,
    mut grid_info: ResMut<GridInfo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let default_mat = materials.add(StandardMaterial::default());

    let window_width = window.width();
    let window_height = window.height();

    // Let this plugin know about grid offset and tile size.
    // This offset and tile size should be same with the value you set for Northstar
    let tile_size = Vec3::splat(32.);
    grid_info.tile_size = tile_size;
    grid_info.grid_offset = Vec3::new(-window_width / 2., -window_height / 2., 0.0);
    let grid_size = UVec2::new(
        (window_width / tile_size.x) as u32,
        (window_height / tile_size.y) as u32,
    );

    // Obstacle
    for i in 0..grid_size.x {
        for j in 0..grid_size.y {
            if i % 2 == 0 && j % 2 == 0 {
                commands.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.5))),
                    MeshMaterial3d(default_mat.clone()),
                    Transform::from_xyz(
                        i as f32 * tile_size.x + grid_info.grid_offset.x,
                        grid_info.grid_offset.y,
                        j as f32 * tile_size.z + grid_info.grid_offset.z,
                    ),
                ));
            }
        }
    }

    // Movement object
    commands.spawn((
        AgentPos(UVec3::ZERO),
        Transform::from_translation(grid_info.grid_offset),
        MouseMovementObject::default(), // Move by mouse input
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(default_mat.clone()),
        LinearMovement {
            speed: 1000.,
            ..default()
        },
    ));

    // Set up Northstar grid
    let grid_settings = GridSettingsBuilder::new_3d(grid_size.x, grid_size.y, 1)
        .chunk_size(8)
        .build();
    let mut grid = CardinalGrid::new(&grid_settings);
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            // Create some staggered impassable cells.
            if x % 2 == 0 && y % 2 == 0 {
                grid.set_nav(UVec3::new(x, y, 0), Nav::Impassable);
            }
        }
    }
    grid.build();
    commands.spawn(grid);

    commands.spawn(Camera3d::default());

    // Ground
    commands.spawn((
        ClickCatcher {
            offset: Vec3::new(0., 0.5 + 0.05, 0.),
        }, // Catch mouse click. Offset = (object height + ground height) / 2
        Mesh3d(meshes.add(Cuboid::new(20., 0.1, 20.))),
        Transform::from_xyz(0.0, 0., 0.0),
        MeshMaterial3d(default_mat.clone()),
    ));

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
