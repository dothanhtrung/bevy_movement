use crate::linear::{LinearDestination, LinearMovement};
#[cfg(feature = "physic")]
use crate::physic::{PhysicDestination, PhysicMovement};
use bevy::app::Update;
use bevy::prelude::{
    in_state, App, ButtonInput, Camera, Component, GlobalTransform, InfinitePlane3d, IntoScheduleConfigs, MouseButton,
    Plugin, Query, Res, States, Window, With,
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
#[derive(Component, Default)]
pub struct MovementObject {
    /// Push new destination to the chain instead of overwrite
    pub is_chain: bool,
}

fn click(
    mouse_btn: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    click_catchers: Query<&GlobalTransform, With<ClickCatcher>>,
    windows: Query<&Window>,
    mut linear_object: Query<(Option<&mut LinearMovement>, &MovementObject)>,
    #[cfg(feature = "physic")] mut physic_object: Query<(Option<&mut PhysicMovement>, &MovementObject)>,
) {
    if mouse_btn.just_pressed(MouseButton::Left) {
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

        for click_catcher in click_catchers.iter() {
            // Calculate if and where the ray is hitting the feeder plane.
            let Some(distance) =
                ray.intersect_plane(click_catcher.translation(), InfinitePlane3d::new(click_catcher.up()))
            else {
                return;
            };
            let point = ray.get_point(distance);

            for (linear_movement, obj) in linear_object.iter_mut() {
                if let Some(mut movement) = linear_movement {
                    let next = LinearDestination::from_pos(point);
                    if obj.is_chain {
                        movement.des.push(next);
                    } else {
                        movement.des = vec![next];
                    }
                }
            }

            #[cfg(feature = "physic")]
            for (physic_movement, obj) in physic_object.iter_mut() {
                if let Some(mut movement) = physic_movement {
                    let next = PhysicDestination::from_pos(point);
                    if obj.is_chain {
                        movement.des.push(next)
                    } else {
                        movement.des = vec![next];
                    }
                }
            }
        }
    }
}
