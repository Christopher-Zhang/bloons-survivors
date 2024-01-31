use bevy::{prelude::*, sprite::collide_aabb::{Collision, collide}}; 
use crate::{data::*};

pub fn _will_collide(pos_a: &Vec3, vel_a: &Velocity, hitbox_a: &Hitbox, pos_b: &Vec3, vel_b: &Velocity, hitbox_b: &Hitbox, delta_time: f32) -> Option<Collision> {
    let new_pos_a = Vec3::new(pos_a.x + vel_a.x * delta_time, pos_a.y + vel_a.y * delta_time, pos_a.z);
    let new_pos_b = Vec3::new(pos_b.x + vel_b.x * delta_time, pos_b.y + vel_b.y * delta_time, pos_b.z);
    collide(new_pos_a, hitbox_a.as_vec2(), new_pos_b, hitbox_b.as_vec2())
}

pub fn _are_colliding(pos_a: &Vec3, hitbox_a: &Hitbox, pos_b: &Vec3, hitbox_b: &Hitbox) -> bool {
    pos_a.x < pos_b.x + hitbox_b.width &&
    pos_a.x + hitbox_a.width > pos_b.x &&
    pos_a.y < pos_b.y + hitbox_b.height &&
    pos_a.y + hitbox_a.height > pos_b.y
}

pub fn _interpolate_velocity (start_vel: &Velocity, target_vel: &Velocity, acceleration: f32) -> Velocity {
    let x = _interpolate_dim(start_vel.x, target_vel.x, acceleration);
    let y = _interpolate_dim(start_vel.y, target_vel.y, acceleration);
    Velocity{x, y, last_dir: Velocity::get_dir(x,y)}
}

pub fn _interpolate_dim(start: f32, target: f32, acceleration: f32) -> f32 {
    let diff = target - start;
    if diff > 0.0 {
        return start + f32::min(acceleration, diff);
    }
    start - f32::min(acceleration, f32::abs(diff))
}

pub fn _debug_move(translation: &Vec3, vel: &Velocity) {
    println!("Moving from {},{} to {},{} with velocity {},{}", translation.x, translation.y, translation.x+vel.x, translation.y+vel.y, vel.x, vel.y);
}

pub fn _signed_angle_between(lhs: Vec3, rhs: Vec3) -> f32 {
    let angle = lhs.angle_between(rhs);
    if lhs.y < rhs.y {
        return -angle;
    }
    angle
}