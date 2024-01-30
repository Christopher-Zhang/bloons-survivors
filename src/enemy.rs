use bevy::{prelude::*, utils::Duration};
use rand::Rng;
use crate::{util::*, data::*};

pub fn spawn_enemy (
    time: Res<Time>, 
    mut spawn_timer: ResMut<EnemySpawnTimer>, 
    // mut rng: ResMut<ThreadRng>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
    // if true {
        let mut rng = rand::thread_rng();
        let x_dist = ENEMY_SPAWN_DIST + rng.gen_range(-ENEMY_SPAWN_WIGGLE..=ENEMY_SPAWN_WIGGLE);
        let y_dist = ENEMY_SPAWN_DIST + rng.gen_range(-ENEMY_SPAWN_WIGGLE..=ENEMY_SPAWN_WIGGLE);
        let rand_vec = Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 0.0);
        let rand_vec = rand_vec.normalize_or_zero() * Vec3::new(x_dist, y_dist, 0.0);
        let sprite_path: &str;
        if rng.gen_bool(0.5) {
            sprite_path = "sprites/forg.png";
        }
        else {
            sprite_path = "sprites/bloon_red.png";
        }
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: rand_vec,
                    scale: DEFAULT_SCALE,
                    ..default()
                },
                texture: asset_server.load(sprite_path),
                ..default()
            },
            Bloon,
            Damage(1.0),
            MaxHealth(1.0),
            CurrentHealth(1.0),
            Hitbox::both(MOB_HITBOX_LENGTH),
            MovementSpeed(ENEMY_MAX_SPEED),
            ZERO_VELOCITY, 
        ));
        // println!("spawning enemy at {:?}", rand_vec);
    }
}

pub fn increase_difficulty (
    time: Res<Time>, mut level_timer: ResMut<EnemyLevelUpTimer>, mut spawn_timer: ResMut<EnemySpawnTimer>
) {
    if level_timer.0.tick(time.delta()).just_finished() {
        let duration = spawn_timer.0.duration().as_secs_f32();
        spawn_timer.0.set_duration(Duration::from_secs_f32(f32::max(duration - DELTA_ENEMY_SPAWN_RATE, MINIMUM_ENEMY_SPAWN_RATE)));
    }
}

pub fn enemy_path (
    mut enemy_query: Query<(&Transform, &mut Velocity, &MovementSpeed), (With<Bloon>, Without<Monkey>)>, player_query: Query<&Transform, (With<Monkey>, Without<Bloon>)>
) {
    let player = player_query.single().translation;
    for (transform, mut vel, walk_speed) in enemy_query.iter_mut() {
        // determine direction and speed
        let speed = walk_speed.0;
        let target_vel = (player - transform.translation).normalize_or_zero() * Vec3::new(speed, speed, 0.0);
        let new_vel = _interpolate_velocity(&vel, &Velocity::from_vec3(&target_vel), ENEMY_ACCEL);
        *vel = new_vel;
    }
}

pub fn enemy_move (
    time: Res<Time>, mut enemy_query: Query<(&mut Transform, &Velocity), With<Bloon>>
) {
    for (mut transform, vel) in enemy_query.iter_mut() {
        // apply movement
        transform.translation.x += vel.x * time.delta_seconds();
        transform.translation.y += vel.y * time.delta_seconds();
    }
}