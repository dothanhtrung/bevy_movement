use bevy::prelude::{Component, Quat, Query, Res, Time, Transform, Vec3};

#[derive(Component)]
pub struct LinearCircleMovement {
    pub speed: f32,
    pub anchor: Vec3,
    pub is_freezed: bool,
    pub axis: Vec3,
}

impl Default for LinearCircleMovement {
    fn default() -> Self {
        Self {
            speed: 0.,
            anchor: Vec3::ZERO,
            is_freezed: false,
            axis: Vec3::Z,
        }
    }
}

pub(crate) fn circle_travel(time: Res<Time>, mut query: Query<(&mut Transform, &LinearCircleMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
        if movement.is_freezed {
            continue;
        }

        let mut offset = transform.translation - movement.anchor;
        let delta_angle = movement.speed * time.delta_secs();
        let rot = Quat::from_axis_angle(movement.axis, delta_angle);
        offset = rot * offset;

        transform.translation = movement.anchor + offset;
    }
}
