use crate::linear::LinearMovement;
use crate::{
    Destination,
    NextDes,
};
use bevy::app::App;
use bevy::math::{
    Vec2,
    Vec3,
};
use bevy::prelude::{
    in_state,
    Commands,
    Component,
    Entity,
    IntoScheduleConfigs,
    Plugin,
    Query,
    Reflect,
    Res,
    States,
    Time,
    Transform,
    Update,
    With,
};
use leafwing_input_manager::prelude::{
    ActionState,
    GamepadStick,
    InputManagerPlugin,
    InputMap,
    VirtualDPad,
};
use leafwing_input_manager::Actionlike;

pub(crate) struct KbControlMovementPlugin<T>
where
    T: States,
{
    states: Vec<T>,
}

impl<T> KbControlMovementPlugin<T>
where
    T: States,
{
    pub(crate) fn new(states: Vec<T>) -> Self {
        Self { states }
    }
}

impl<T> Plugin for KbControlMovementPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MovementAction>::default())
            .add_systems(Update, builder);
        if self.states.is_empty() {
            app.add_systems(Update, moving);
        } else {
            for state in &self.states {
                app.add_systems(Update, moving.run_if(in_state(state.clone())));
            }
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MovementAction {
    #[actionlike(DualAxis)]
    Walk,
}

#[derive(Component, Default)]
struct ActionInit;

#[derive(Component)]
#[require(ActionInit)]
pub struct KbMovementObject {
    /// Internal use for store moving state. You should not change this value.
    pub is_moving: bool,
    /// Gamepad button to control movement. Default `GamepadStick::LEFT`.
    pub gamepad: Vec<GamepadStick>,
    /// DPad button to control movement. Default `VirtualDPad::wasd()` & `VirtualDPad::arrow_keys()`.
    pub dpad: Vec<VirtualDPad>,
}

impl Default for KbMovementObject {
    fn default() -> Self {
        Self {
            is_moving: false,
            gamepad: vec![GamepadStick::LEFT],
            dpad: vec![VirtualDPad::wasd(), VirtualDPad::arrow_keys()],
        }
    }
}

fn builder(mut commands: Commands, query: Query<(Entity, &KbMovementObject), With<ActionInit>>) {
    for (entity, kb_object) in query.iter() {
        let mut input_map = InputMap::default();

        for input in kb_object.gamepad.iter() {
            input_map.insert_dual_axis(MovementAction::Walk, input.clone());
        }
        for input in kb_object.dpad.iter() {
            input_map.insert_dual_axis(MovementAction::Walk, input.clone());
        }

        commands.entity(entity).insert(input_map);
        commands.entity(entity).remove::<ActionInit>();
    }
}

fn moving(
    mut commands: Commands,
    mut query: Query<(
        &ActionState<MovementAction>,
        &mut LinearMovement,
        &Transform,
        &mut KbMovementObject,
        Entity,
    )>,
    time: Res<Time>,
) {
    for (state, mut movement, transform, mut kb_control, entity) in query.iter_mut() {
        if state.axis_pair(&MovementAction::Walk) != Vec2::ZERO {
            kb_control.is_moving = true;
            let direction = state.clamped_axis_pair(&MovementAction::Walk);
            // Make a distance litter further than what object can travel in 1 tick
            let distance = movement.speed * time.delta_secs() * 2.;

            let next_pos = if cfg!(feature = "2d") {
                Vec3::new(
                    transform.translation.x + direction.x * distance,
                    transform.translation.y + direction.y * distance,
                    transform.translation.z,
                )
            } else {
                Vec3::new(
                    transform.translation.x + direction.x * distance,
                    transform.translation.y,
                    transform.translation.z - direction.y * distance,
                )
            };
            if let Some(last_pos) = movement.des.last() {
                if last_pos.pos == next_pos {
                    continue;
                }
            }
            commands.trigger(NextDes {
                entity,
                des: Destination::from_pos(next_pos),
                is_chain: false,
            });
        } else if kb_control.is_moving {
            movement.stop();
            kb_control.is_moving = false;
        }
    }
}
