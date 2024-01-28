use bevy::{a11y::accesskit::TextAlign, prelude::*, render::camera::ScalingMode, text, time::Stopwatch, utils::Duration};
use rand::Rng;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Alive,
    Paused,
    Dead
}
const BACKGROUND_COLOR: Color = Color::rgb(0.3, 0.6, 0.3);
const DEFAULT_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const DEFAULT_SCALE: Vec3 = Vec3::new(3.0, 3.0, 1.0);
const PLAYER_MAX_SPEED: f32 = 500.0;
const PLAYER_ACCEL: f32 = 200.0;
const ENEMY_MAX_SPEED: f32 = 300.0;
const ENEMY_ACCEL: f32 = 50.0;
const ENEMY_SPAWN_DIST: f32 = 2000.0;
const ENEMY_SPAWN_WIGGLE: f32 = 300.0;
const INITIAL_ENEMY_SPAWN_RATE: f32 = 1.0;
const DELTA_ENEMY_SPAWN_RATE: f32 = 0.1;
const MINIMUM_ENEMY_SPAWN_RATE: f32 = 0.2;
const ZERO_VELOCITY: Velocity = Velocity{x: 0.0, y: 0.0};
const PLAYER_STARTING_HEALTH: f32 = 10.0;
const SPRITE_WIDTH: f32 = 32.0;
const SPRITE_HEIGHT: f32 = 32.0;
const DEFAULT_HITBOX_WIDTH: f32 = 16.0;
const DEFAULT_HITBOX_HEIGHT: f32 = 16.0;
const HEALTHBAR_COLOR: Color = Color::CRIMSON;
const HEALTHBAR_HEIGHT: f32 = 10.0;
enum WeaponPath {
    Line
}
#[derive(Component)]
struct Weapon {
    sprite_path: String,
    base_damage: f32,
    damage_modifier: f32,
    pierce: u32,
    missile_life: f32,
    missile_speed: f32,
    weapon_path: WeaponPath
}
#[derive(Component)]
struct Projectile {
    pierce: u32,
    damage: f32,
    missile_life: f32,
    missile_speed: f32,
}
#[derive(Component)]
struct Healthbar;
#[derive(Component)]
struct Monkey;
// MaxHealth
// CurrentHealth
// MovementSpeed
#[derive(Component)]
struct Bloon;
// Damage
// MaxHealth
// CurrentHealth
// MovementSpeed
#[derive(Component, Clone, Copy, Debug)]
struct Velocity { x: f32, y: f32 }

impl Velocity {
    fn from_vec3 (v: &Vec3) -> Velocity {
        Velocity {x: v.x, y: v.y}
    } 
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}
#[derive(Component, Clone)]
struct MovementSpeed(f32);
#[derive(Component, Clone)]
struct CurrentHealth(f32);
#[derive(Component, Clone)]
struct MaxHealth(f32);
#[derive(Component, Clone)]
struct Damage(f32);
#[derive(Component, Clone)]
struct Hitbox {
    width: f32,
    height: f32,
}
#[derive(Component)]
struct CollisionFlags(Vec<usize>);
#[derive(Resource)]
struct MyTimer(Timer);
#[derive(Resource)]
struct HeartbeatTimer(Timer);
#[derive(Resource)]
struct EnemySpawnTimer(Timer);
#[derive(Resource)]
struct EnemyLevelUpTimer(Timer);
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
            println!("damage {} -> {}", player_health.0, player_health.0 - enemy_damage.0);
            player_health.0 -= enemy_damage.0;

        }
    }
}

fn _will_collide(pos_a: &Vec3, vel_a: &Velocity, hitbox_a: &Hitbox, pos_b: &Vec3, vel_b: &Velocity, hitbox_b: &Hitbox, delta_time: f32) -> bool {
    let new_pos_a = Vec3::new(pos_a.x + vel_a.x * delta_time, pos_a.y + vel_a.y * delta_time, pos_a.z);
    let new_pos_b = Vec3::new(pos_b.x + vel_b.x * delta_time, pos_b.y + vel_b.y * delta_time, pos_b.z);
    _are_colliding(&new_pos_a, hitbox_a, &new_pos_b, hitbox_b)
}

fn _are_colliding(pos_a: &Vec3, hitbox_a: &Hitbox, pos_b: &Vec3, hitbox_b: &Hitbox) -> bool {
    pos_a.x < pos_b.x + hitbox_b.width &&
    pos_a.x + hitbox_a.width > pos_b.x &&
    pos_a.y < pos_b.y + hitbox_b.height &&
    pos_a.y + hitbox_a.height > pos_b.y
}

// fn 

fn spawn_enemy (
    time: Res<Time>, 
    mut spawn_timer: ResMut<EnemySpawnTimer>, 
    // mut rng: ResMut<ThreadRng>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    // if spawn_timer.0.tick(time.delta()).just_finished() {
    if true {
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
            MaxHealth(2.0),
            CurrentHealth(2.0),
            Hitbox {
                width: DEFAULT_HITBOX_WIDTH,
                height: DEFAULT_HITBOX_HEIGHT
            },
            MovementSpeed(ENEMY_MAX_SPEED),
            ZERO_VELOCITY, 
        ));
        println!("spawning enemy at {:?}", rand_vec);
    }
}

fn increase_difficulty (
    time: Res<Time>, mut level_timer: ResMut<EnemyLevelUpTimer>, mut spawn_timer: ResMut<EnemySpawnTimer>
) {
    if level_timer.0.tick(time.delta()).just_finished() {
        let duration = spawn_timer.0.duration().as_secs_f32();
        spawn_timer.0.set_duration(Duration::from_secs_f32(f32::max(duration - DELTA_ENEMY_SPAWN_RATE, MINIMUM_ENEMY_SPAWN_RATE)));
    }
}

fn enemy_path (
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

fn enemy_move (
    time: Res<Time>, mut enemy_query: Query<(&mut Transform, &Velocity), With<Bloon>>
) {
    for (mut transform, vel) in enemy_query.iter_mut() {
        // apply movement
        transform.translation.x += vel.x * time.delta_seconds();
        transform.translation.y += vel.y * time.delta_seconds();
    }
}

fn handle_input(
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
        let target_vel = Velocity{x,y};
        let new_vel = _interpolate_velocity(&vel, &target_vel, PLAYER_ACCEL);
        // println!("start vel: {:?}, target vel {:?}, new_vel {:?}", vel, target_vel, new_vel);
        *vel = new_vel;
    }
}

fn handle_movement (
    time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Monkey>>
) {
    let (mut player_transform, vel) = query.single_mut();
    player_transform.translation.x += vel.x * time.delta_seconds();
    player_transform.translation.y += vel.y * time.delta_seconds();
    // println!("Player moved to {},{} with velocity {},{}", player_transform.translation.x,player_transform.translation.y,vel.x,vel.y);
}

fn _interpolate_velocity (start_vel: &Velocity, target_vel: &Velocity, acceleration: f32) -> Velocity {
    Velocity{x: _interpolate_dim(start_vel.x, target_vel.x, acceleration), y: _interpolate_dim(start_vel.y, target_vel.y, acceleration)}
}

fn _interpolate_dim(start: f32, target: f32, acceleration: f32) -> f32 {
    let diff = target - start;
    if diff > 0.0 {
        return start + f32::min(acceleration, diff);
    }
    start - f32::min(acceleration, f32::abs(diff))
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
    commands.spawn((
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
        Hitbox {
            width: DEFAULT_HITBOX_WIDTH,
            height: DEFAULT_HITBOX_HEIGHT
        },
        ZERO_VELOCITY, 
    ));
    
    
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

            enemy_path,
            player_enemy_collision,
            handle_movement,
            enemy_move,
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