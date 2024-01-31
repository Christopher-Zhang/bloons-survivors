use bevy::prelude::*;
use crate::{util::*, data::*};

pub fn handle_movement (
    time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Monkey>>
) {
    let (mut player_transform, vel) = query.single_mut();
    player_transform.translation.x += vel.x * time.delta_seconds();
    player_transform.translation.y += vel.y * time.delta_seconds();
    if vel.x != 0. || vel.y != 0. {
        let angle = vel.y.atan2(vel.x);
        let rotate = Quat::from_rotation_z(angle);
        player_transform.rotation = rotate;
    }
    // println!("Player moved to {},{} with velocity {},{}", player_transform.translation.x,player_transform.translation.y,vel.x,vel.y);
}