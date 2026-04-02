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
    States,
    Update,
    With,
};
use leafwing_input_manager::prelude::{
    ActionState,
    GamepadStick,
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

        input_map
    }
}

// #[derive(Component)]
// struct ActionInit;
//
// #[derive(Component)]
// #[require(ActionInit)]
// pub struct KbMovementObject;
//
// fn builder(mut commands: Commands, query: Query<(Entity, Option<&LinearMovement>), With<ActionInit>>) {
//     for (entity, movement) in query.iter() {
//         commands.entity(entity).insert(MovementAction::new());
//         commands.entity(entity).remove::<ActionInit>();
//     }
// }

fn moving(mut query: Query<(ActionState<MovementAction>, &mut LinearMovement)>) {
    for (state, mut movement) in query.iter_mut() {
        if state.axis_pair(&MovementAction::Walk) != Vec2::ZERO {
            movement.des.push(LinearDestination::from_pos(
                state.clamped_axis_pair(&MovementAction::Walk).into(),
            ));
        }
    }
}
