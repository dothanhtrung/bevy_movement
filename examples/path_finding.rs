use bevy::color::palettes::basic::{
    RED,
    WHITE,
};
use bevy::prelude::*;
use bevy_movement::linear::{
    GridInfo,
    LinearMovement,
};
use bevy_movement::mouse_control::MouseMovementObject;
use bevy_movement::MovementPluginAnyState;
use bevy_northstar::components::AgentPos;
use bevy_northstar::plugin::NorthstarPlugin;
use bevy_northstar::prelude::{
    CardinalNeighborhood,
    GridSettingsBuilder,
    Nav,
    NorthstarDebugPlugin,
};
use bevy_northstar::CardinalGrid;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins((
        NorthstarPlugin::<CardinalNeighborhood>::default(),
        NorthstarDebugPlugin::<CardinalNeighborhood>::default(),
    ));

    app.add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, window: Single<&Window>, mut grid_info: ResMut<GridInfo>) {
    let window_width = window.width();
    let window_height = window.height();
    grid_info.tile_size = Vec3::splat(32.);
    let tile_size = grid_info.tile_size;
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
                    Sprite {
                        color: WHITE.into(),
                        custom_size: Some(Vec2::new(tile_size.x, tile_size.y)),
                        ..default()
                    },
                    Transform::from_xyz(
                        i as f32 * tile_size.x + grid_info.grid_offset.x,
                        j as f32 * tile_size.y + grid_info.grid_offset.y,
                        0.,
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
        Sprite {
            color: RED.into(),
            custom_size: Some(Vec2::new(tile_size.x, tile_size.y)),
            ..default()
        },
        LinearMovement {
            speed: 1000.,
            ..default()
        },
    ));

    // Spawn grid
    let grid_settings = GridSettingsBuilder::new_2d(grid_size.x, grid_size.y)
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

    commands.spawn(Camera2d);
}
