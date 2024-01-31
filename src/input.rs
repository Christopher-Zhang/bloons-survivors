use bevy::{prelude::*, window::PrimaryWindow};
use crate::{util::*, data::*};

pub fn handle_keyboard_input(
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

pub fn handle_mouse_input(
    mut mouse_query: Query<(&mut OnScreen, &mut Position), With<Mouse>>, 
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    let (camera, camera_transform) = query_camera.single();
    let (mut on_screen, mut pos) = mouse_query.single_mut();
    println!("check mouse");
    if let Some(position) = query_window.single().cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()
    ) {
        on_screen.0 = true;
        pos.0 = position;
        // dbg!(&pos.0);
        print!(".");
    }
    else {
        println!("mouse not on screen");
        on_screen.0 = false;
    }
}

