use crate::linear::{
    LinearDestination,
    LinearMovement,
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
    Window,
    With,
};

macro_rules! mouse_control_movement_systems {
    () => {
        (click)
    };
}

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
            app.add_systems(Update, mouse_control_movement_systems!());
        } else {
            for state in &self.states {
                app.add_systems(
                    Update,
                    mouse_control_movement_systems!().run_if(in_state(state.clone())),
                );
            }
        }
    }
}

#[derive(Component)]
pub struct ClickCatcher;

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
    click_catchers: Query<&GlobalTransform, With<ClickCatcher>>,
    windows: Query<&Window>,
    mut linear_object: Query<(Entity, &mut LinearMovement, &MouseMovementObject)>,
) {
    for (entity, mut linear_movement, obj) in linear_object.iter_mut() {
        if mouse_btn.any_just_pressed(obj.click_button.clone()) {
            let Ok((camera, camera_transform)) = camera_query.single() else {
                return;
            };

            let Ok(window) = windows.single() else {
                return;
            };
            let Some(cursor_position) = window.cursor_position() else {
                return;
            };

            // Calculate a ray pointing from the camera into the world based on the cursor's position.
            let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
                return;
            };

            if let Ok(click_catcher) = click_catchers.single() {
                // Calculate if and where the ray is hitting the feeder plane.
                let Some(distance) =
                    ray.intersect_plane(click_catcher.translation(), InfinitePlane3d::new(click_catcher.up()))
                else {
                    return;
                };
                let point = ray.get_point(distance);
                commands.trigger(NextDes { entity, pos: point });

                if mouse_btn.any_just_pressed(obj.click_button.clone()) {
                    let next = LinearDestination::from_pos(point);
                    if obj.is_chain {
                        linear_movement.des.push(next);
                    } else {
                        linear_movement.des = vec![next];
                    }
                }
            }
        }
    }
}
