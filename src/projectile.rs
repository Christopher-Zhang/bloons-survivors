use bevy::prelude::*;
use crate::{data::*, util::*};


// pub struct WeaponPath {
//     starting_angles: Vec<f32>,
//     starting_distances: Vec<f32>,
//     trajectory: Trajectory,
// }
// #[derive(Component)]
// pub struct Weapon {
//     pub sprite_path: String,
//     pub base_damage: f32,
//     pub damage_modifier: f32,
//     pub pierce: u32,
//     pub missile_life: f32,
//     pub missile_speed: f32,
//     pub weapon_path: WeaponPath,
//     pub base_attack_speed: u32,
//     pub frames_left: u32,
// }

pub fn add_weapon (
    weapon: Weapon, player_id: Entity , commands: &mut Commands,
) {
    commands.entity(player_id)
        .insert(weapon);
}

pub fn fire_projectiles (
    mut weapon_query: Query<(&mut Weapon, &Transform, &Velocity), With<Monkey>>, mut commands: Commands, asset_server: Res<AssetServer>
) {
    for (mut weapon, player_transform, player_vel) in weapon_query.iter_mut() {
        let player_position = player_transform.translation;
        if weapon.frames_left == 0 {
            weapon.frames_left = weapon.base_attack_speed;
            let sprite = asset_server.load(&weapon.sprite_path);
            for starting_position in std::iter::zip(
                weapon.weapon_path.starting_angles.iter(), 
                weapon.weapon_path.starting_distances.iter()
            ) {
                // TODO decide the starting position and angle based on the given params
                // maybe set up a trajectory
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: player_position,
                            scale: DEFAULT_SCALE,
                            ..default()
                        },
                        texture: sprite.clone(),
                        ..default()
                    },
                    Projectile::new(
                        weapon.pierce, 
                        weapon.base_damage * weapon.damage_modifier,
                        weapon.missile_life,
                        weapon.missile_speed,
                        // _signed_angle_between(player_vel.last_dir, DEFAULT_DIR)
                        player_vel.last_dir.y.atan2(player_vel.last_dir.x)
                    ),
                    Velocity::new(0.0, 0.0),
                    Hitbox::both(PROJECTILE_HITBOX_LENGTH),
                ));
                // println!("angle = {:?}, trig: {:?}", player_vel.last_dir.angle_between(DEFAULT_DIR), Vec2::from_angle(player_vel.last_dir.angle_between(DEFAULT_DIR)));
            }
        }
        else {
            weapon.frames_left -= 1;
        }
    }
}

pub fn projectile_path (
    mut query: Query<(&mut Velocity, &mut Projectile, Entity, &mut Transform), With<Projectile>>, mut commands: Commands
) {
    for (mut vel, mut projectile, entity, mut transform) in query.iter_mut() {
        if projectile.pierce == 0 || projectile.missile_life == 0{
            commands.entity(entity).despawn();
            continue;
        }
        let trig = Vec2::from_angle(projectile.angle);
        vel.x = projectile.speed * trig.x;
        vel.y = projectile.speed * trig.y;
        let rotate = Quat::from_rotation_z(projectile.angle);
        transform.rotation = rotate;

        projectile.missile_life -= 1;
    }
}

pub fn projectile_move (
    time: Res<Time>, mut query: Query<(&Velocity, &mut Transform), With<Projectile>>
) {
    for (vel, mut transform) in query.iter_mut() {
        // _debug_move(&transform.translation, &vel);
        transform.translation.x += vel.x * time.delta_seconds();
        transform.translation.y += vel.y * time.delta_seconds();
    }
}