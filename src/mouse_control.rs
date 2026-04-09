use crate::linear::{
    LinearDestination,
    LinearMovement,
    GridInfo,
};
use crate::NextDes;
use bevy::app::Update;
use bevy::prelude::{
    in_state,
    App,
    ButtonInput,
    Camera,
    Commands,
    Component,
    Entity,
    GlobalTransform,
    InfinitePlane3d,
    IntoScheduleConfigs,
    MouseButton,
    Plugin,
    Query,
    Res,
    States,
    Vec3,
    Window,
    Without,
};
#[cfg(feature = "path_finding")]
use bevy_northstar::prelude::Pathfind;

pub(crate) struct MouseControlMovementPlugin<T>
where
    T: States,
{
    states: Vec<T>,
}

impl<T> MouseControlMovementPlugin<T>
where
    T: States,
{
    pub(crate) fn new(states: Vec<T>) -> Self {
        Self { states }
    }
}

impl<T> Plugin for MouseControlMovementPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        if self.states.is_empty() {
            app.add_systems(Update, click);
        } else {
            for state in &self.states {
                app.add_systems(Update, click.run_if(in_state(state.clone())));
            }
        }
    }
}

#[derive(Component, Default)]
pub struct ClickCatcher {
    pub offset: Vec3,
}

#[derive(Component)]
pub struct MouseMovementObject {
    /// Push new destination to the chain instead of overwrite
    pub is_chain: bool,

    /// Which button will trigger movement. Default is MouseButton::Left.
    pub click_button: Vec<MouseButton>,
}

impl Default for MouseMovementObject {
    fn default() -> Self {
        Self {
            is_chain: false,
            click_button: vec![MouseButton::Left],
        }
    }
}

fn click(
    mut commands: Commands,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    click_catchers: Query<(&GlobalTransform, &ClickCatcher), Without<Camera>>,
    windows: Query<&Window>,
    mut linear_object: Query<(Entity, &mut LinearMovement, &MouseMovementObject)>,
    #[cfg(feature = "path_finding")] tile_size: Res<GridInfo>,
) {
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let mut world_pos = Vec3::ZERO;
    if cfg!(feature = "2d") {
        let Ok(world_pos_2d) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
            return;
        };

        world_pos = Vec3::new(world_pos_2d.x, world_pos_2d.y, 0.);
    } else {
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        for (global_transform, click_catcher) in click_catchers.iter() {
            // Calculate if and where the ray is hitting the feeder plane.
            let Some(distance) = ray.intersect_plane(
                global_transform.translation(),
                InfinitePlane3d::new(global_transform.up()),
            ) else {
                continue;
            };
            world_pos = ray.get_point(distance) + click_catcher.offset;
            break;
        }
    }

    for (entity, mut linear_movement, obj) in linear_object.iter_mut() {
        if mouse_btn.any_just_pressed(obj.click_button.clone()) {
            commands.trigger(NextDes { entity, pos: world_pos });

            if cfg!(not(feature = "path_finding")) {
                let next = LinearDestination::from_pos(world_pos);
                if obj.is_chain {
                    linear_movement.des.push(next);
                } else {
                    linear_movement.des = vec![next];
                }
            }

            #[cfg(feature = "path_finding")]
            {
                let tile_pos = (world_pos - tile_size.grid_offset) / tile_size.tile_size;
                commands.entity(entity).insert(Pathfind::new(tile_pos.as_uvec3()));
            }
        }
    }
}
