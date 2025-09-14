use bevy::prelude::{Query, Res, Time, Transform, Vec3, Quat, Component};

#[derive(Component, Default)]
pub struct LinearCircleMovement {
    pub speed: f32,
    pub anchor: Vec3,
    pub is_freezed: bool,
    pub axis: Vec3,
}

pub(crate) fn circle_travel(time: Res<Time>, mut query: Query<(&mut Transform, &LinearCircleMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
        if movement.is_freezed {
            continue;
        }

        let mut offset = transform.translation - movement.anchor;
        //
        // // If offset is degenerate or not at the desired radius, initialize/adjust it
        // let len_sq = offset.length_squared();
        // if len_sq < 1e-8 {
        //     // Build a perpendicular vector to axis as starting offset
        //     let helper = if axis.dot(Vec3::Y).abs() < 0.99 { Vec3::Y } else { Vec3::X };
        //     let right = axis.cross(helper).normalize_or_zero();
        //     offset = if right.length_squared() > 0.0 { right * movement.radius } else { Vec3::X * movement.radius };
        // } else {
        //     offset = offset.normalize() * movement.radius;
        // }

        // Rotate the offset around the axis by delta angle
        let delta_angle = movement.speed * time.delta_secs();
        let rot = Quat::from_axis_angle(movement.axis, delta_angle);
        offset = rot * offset;

        transform.translation = movement.anchor + offset;
    }
}