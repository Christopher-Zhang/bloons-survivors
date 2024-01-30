use bevy::prelude::*;
use crate::{util::*, data::*};

pub fn handle_input(
    keys: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &MovementSpeed), With<Monkey>>
) {
    for (mut vel, speed) in query.iter_mut() {
        let mut x: f32= 0.0;
        let mut y: f32 = 0.0;
        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            y += 1.0;
        }
        else if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
            y -= 1.0;
        }
        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            x -= 1.0;
        }
        else if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
            x += 1.0;
        }
        if x != 0.0 && y != 0.0 {
            let mag = (x*x + y*y).sqrt();
            x /= mag;
            y /= mag;
        }
        x *= speed.0;
        y *= speed.0;
        let target_vel = Velocity::new(x,y);
        let new_vel = _interpolate_velocity(&vel, &target_vel, PLAYER_ACCEL);
        // println!("start vel: {:?}, target vel {:?}, new_vel {:?}", vel, target_vel, new_vel);
        vel.set(new_vel.x, new_vel.y);
    }
}

pub fn handle_movement (
    time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Monkey>>
) {
    let (mut player_transform, vel) = query.single_mut();
    player_transform.translation.x += vel.x * time.delta_seconds();
    player_transform.translation.y += vel.y * time.delta_seconds();
    // println!("Player moved to {},{} with velocity {},{}", player_transform.translation.x,player_transform.translation.y,vel.x,vel.y);
}