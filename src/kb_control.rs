use crate::linear::{
    LinearDestination,
    LinearMovement,
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

impl MovementAction {
    fn new() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_dual_axis(Self::Walk, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Walk, VirtualDPad::wasd());
        input_map.insert_dual_axis(Self::Walk, VirtualDPad::arrow_keys());

        input_map
    }
}

#[derive(Component, Default)]
struct ActionInit;

// TODO: Allow specify key for action
#[derive(Component)]
#[require(ActionInit)]
pub struct KbMovementObject {
    is_moving: bool,
}

impl KbMovementObject {
    pub fn new() -> Self {
        Self { is_moving: false }
    }
}

fn builder(mut commands: Commands, query: Query<Entity, With<ActionInit>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(MovementAction::new());
        commands.entity(entity).remove::<ActionInit>();
    }
}

fn moving(
    mut query: Query<(
        &ActionState<MovementAction>,
        &mut LinearMovement,
        &Transform,
        &mut KbMovementObject,
    )>,
    time: Res<Time>,
) {
    for (state, mut movement, transform, mut kb_control) in query.iter_mut() {
        if state.axis_pair(&MovementAction::Walk) != Vec2::ZERO {
            kb_control.is_moving = true;
            let direction = state.clamped_axis_pair(&MovementAction::Walk);
            let distance = movement.speed * time.delta_secs();

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
            movement.des = vec![LinearDestination::from_pos(next_pos)];
        } else if kb_control.is_moving {
            movement.stop();
            kb_control.is_moving = false;
        }
    }
}
