use bevy::{a11y::accesskit::TextAlign, prelude::*, render::camera::ScalingMode, text, time::Stopwatch, utils::Duration};
use rand::Rng;
use crate::{
    data::*, 
    util::{_are_colliding, _will_collide, _interpolate_dim, _interpolate_velocity},
    enemy::*,
    player::*,
    projectile::*,
};
mod data;
mod util;
mod player;
mod enemy;
mod projectile;

// #[derive(Resource)]
// struct WorldClock(Stopwatch);
// #[derive(Resource)]
// struct Rng(ThreadRng);
fn player_enemy_collision(mut player_query: Query<(&Transform, &Hitbox, &mut CurrentHealth), With<Monkey>>, enemy_query: Query<(&Transform, &Hitbox, &Damage), With<Bloon>>) {
    let (player_pos, player_hitbox, mut player_health) = player_query.single_mut();
    for (enemy_pos, enemy_hitbox, enemy_damage) in enemy_query.iter() {
        // check collision
        if _are_colliding(&player_pos.translation, &player_hitbox, &enemy_pos.translation, &enemy_hitbox) {
            // TODO
                // think about consecutive collisions, should not damage every frame
            // apply damage if necessary
            // println!("damage {} -> {}", player_health.0, player_health.0 - enemy_damage.0);
            player_health.0 -= enemy_damage.0;

        }
    }
}

pub fn projectile_enemy_collision(
    mut enemy_query: Query<(&mut CurrentHealth, &Transform, &Hitbox, Entity), With<Bloon>>, mut projectile_query: Query<(&mut Projectile, &Transform, &Hitbox, Entity), With<Projectile>>, mut commands: Commands
) {
    for (mut projectile, projectile_transform, projectile_hitbox, projectile_entity) in projectile_query.iter_mut() {
        for (mut current_health, enemy_transform, enemy_hitbox, enemy_entity) in enemy_query.iter_mut() {
            if _are_colliding(&projectile_transform.translation, &projectile_hitbox, &enemy_transform.translation, &enemy_hitbox) {
                current_health.0 -= projectile.damage;
                projectile.pierce -= 1;
                if projectile.pierce <= 0 {
                    commands.entity(projectile_entity).despawn();
                }
                if current_health.0 <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
    // detect collision
    // check collision flags
    // update enemy health
    // if die, should drop stuff
}

fn update_ui (
    player_query: Query<&CurrentHealth, With<Monkey>>, mut healthbar_query: Query<&mut Text, With<Healthbar>>
) {
    let mut text = healthbar_query.single_mut();
    let current_health = player_query.single();
    text.sections[0].value = format!("Health: {}", current_health.0);
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>
) {
    // camera
    let mut my_camera = Camera2dBundle::default();
    my_camera.projection.scaling_mode = ScalingMode::FixedVertical(1600.0);
    commands.spawn(my_camera);
    
    // player
    let player_sprite = asset_server.load("sprites/player.png");
    let player_id = commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: DEFAULT_POS,
                scale: DEFAULT_SCALE,
                ..default()
            },
            texture: player_sprite,
            ..default()
        },
        Monkey,
        MaxHealth(PLAYER_STARTING_HEALTH),
        CurrentHealth(PLAYER_STARTING_HEALTH),
        MovementSpeed(PLAYER_MAX_SPEED),
        Hitbox::both(MOB_HITBOX_LENGTH),
        ZERO_VELOCITY, 
    )).id();
    
    let starting_weapon = Weapon::new(
        String::from("sprites/dart.png"),
        1.0,
        1.0,
        3,
        120,
        1000.0,
        WeaponPath::new(
            vec![0.0],
            vec![0.0],
            Trajectory::Line
        ),
        20,
        20
    );
    add_weapon(starting_weapon, player_id, &mut commands);
    
    // UI
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 64.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(&format!("Health: {PLAYER_STARTING_HEALTH}"), text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        Healthbar
    ));
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .insert_resource(MyTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
    .insert_resource(HeartbeatTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
    .insert_resource(EnemySpawnTimer(Timer::from_seconds(INITIAL_ENEMY_SPAWN_RATE, TimerMode::Repeating)))
    .insert_resource(EnemyLevelUpTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
    .add_systems(Startup, setup)
    .add_systems(
        FixedUpdate, ( 
            bevy::window::close_on_esc,
            handle_input,
            fire_projectiles,
            projectile_path,
            projectile_move,
            enemy_path,
            player_enemy_collision,
            handle_movement,
            enemy_move,
            projectile_enemy_collision,
            spawn_enemy,
            increase_difficulty,
            update_ui,
        ).chain()
    )
    .run();
}
// fn heartbeat(time: Res<Time>, mut timer: ResMut<HeartbeatTimer>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         println!("boop!");
//     }
// }

// fn ping_all_monkeys(time: Res<Time>, mut timer: ResMut<MyTimer>, query: Query<Entity, With<Monkey>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for id in &query {
//             println!("ping {}", id.index());
//         }
//     }
// }