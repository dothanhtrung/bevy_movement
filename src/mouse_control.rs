#[cfg(feature = "path_finding")]
use crate::linear::GridInfo;
use crate::{
    Arrived,
    Destination,
    NextDes,
};
use bevy::app::Update;
#[cfg(feature = "path_finding")]
use bevy::prelude::UVec3;
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
    On,
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
        app.add_observer(next_des).add_observer(arrived);

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

    pub goals: Vec<Vec3>,

    /// Which button will trigger movement. Default is MouseButton::Left.
    pub click_button: Vec<MouseButton>,
}

impl Default for MouseMovementObject {
    fn default() -> Self {
        Self {
            is_chain: false,
            goals: Vec::new(),
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
    mut linear_object: Query<(Entity, &mut MouseMovementObject)>,
    #[cfg(feature = "path_finding")] grid_info: Res<GridInfo>,
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

    for (entity, mut mv_object) in linear_object.iter_mut() {
        if mouse_btn.any_just_pressed(mv_object.click_button.clone()) {
            if !mv_object.is_chain {
                mv_object.goals.clear();
            }
            mv_object.goals.push(world_pos);

            if cfg!(not(feature = "path_finding")) {
                let is_chain = mv_object.is_chain;
                let next_des = NextDes {
                    entity,
                    des: Destination::from_pos(world_pos),
                    is_chain,
                };
                commands.trigger(next_des);
            }

            #[cfg(feature = "path_finding")]
            {
                commands
                    .entity(entity)
                    .insert(Pathfind::new(world_pos_to_tile(&world_pos, &grid_info)));
            }
        }
    }
}

fn arrived(
    trigger: On<Arrived>,
    mut _commands: Commands,
    mut query: Query<(&mut MouseMovementObject, Entity)>,
    #[cfg(feature = "path_finding")] grid_info: Res<GridInfo>,
) {
    if let Ok((mut mv_obj, _entity)) = query.get_mut(trigger.entity) {
        if !mv_obj.goals.is_empty() && *mv_obj.goals.first().unwrap() == trigger.pos {
            mv_obj.goals.remove(0);
        }

        #[cfg(feature = "path_finding")]
        {
            if let Some(world_pos) = mv_obj.goals.first() {
                _commands
                    .entity(_entity)
                    .insert(Pathfind::new(world_pos_to_tile(world_pos, &grid_info)));
            }
        }
    }
}

fn next_des(trigger: On<NextDes>, mut query: Query<&mut MouseMovementObject>) {
    if let Ok(mut mv_obj) = query.get_mut(trigger.entity) {
        if mv_obj.goals.is_empty() {
            return;
        }

        // Remove all goals if canceled
        if !trigger.is_chain && trigger.des.pos != *mv_obj.goals.first().unwrap() {
            mv_obj.goals.clear();
        }
    }
}

#[cfg(feature = "path_finding")]
fn world_pos_to_tile(pos: &Vec3, grid_info: &GridInfo) -> UVec3 {
    let tile_pos = (pos - grid_info.grid_offset) / grid_info.tile_size;
    UVec3::new(
        tile_pos.x.round() as u32,
        tile_pos.y.round() as u32,
        tile_pos.z.round() as u32,
    )
}
