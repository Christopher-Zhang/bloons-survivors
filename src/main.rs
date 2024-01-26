use bevy::{prelude::*, time::Stopwatch, utils::Duration};

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
const INITIAL_ENEMY_SPAWN_RATE: f32 = 5.0;
const DELTA_ENEMY_SPAWN_RATE: f32 = 0.1;
const MINIMUM_ENEMY_SPAWN_RATE: f32 = 0.2;
const ZERO_VELOCITY: Velocity = Velocity{x: 0.0, y: 0.0};

#[derive(Component)]
struct Monkey;
#[derive(Component)]
struct Bloon;
#[derive(Component)]
struct Id(u32);
#[derive(Component, Clone, Copy, Debug)]
struct Velocity { x: f32, y: f32 }
#[derive(Component, Clone)]
struct WalkSpeed(f32);

#[derive(Resource)]
struct MyTimer(Timer);
#[derive(Resource)]
struct HeartbeatTimer(Timer);
#[derive(Resource)]
struct EnemySpawnTimer(Timer);
#[derive(Resource)]
struct EnemyLevelUpTimer(Timer);
#[derive(Resource)]
struct WorldClock(Stopwatch);



fn heartbeat(time: Res<Time>, mut timer: ResMut<HeartbeatTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("boop!");
    }
}
fn ping_all_monkeys(time: Res<Time>, mut timer: ResMut<MyTimer>, query: Query<&Id, With<Monkey>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for id in &query {
            println!("ping {}", id.0);
        }
    }
}
fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>
) {
    
    commands.spawn(Camera2dBundle::default());


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
        Id(0), 
        ZERO_VELOCITY, 
        WalkSpeed(PLAYER_MAX_SPEED)
    ));
}

fn spawn_enemy (
    time: Res<Time>, mut spawn_timer: ResMut<EnemySpawnTimer>, mut commands: Commands, asset_server: Res<AssetServer>
) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: DEFAULT_POS,
                    scale: DEFAULT_SCALE,
                    ..default()
                },
                texture: asset_server.load("sprites/blood_red.png"),
                ..default()
            },
            Bloon, 
            Id(0), 
            ZERO_VELOCITY, 
            WalkSpeed(ENEMY_MAX_SPEED)
        ));
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

fn enemy_move (
    time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Monkey>>
) {

}

fn handle_input(
    keys: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Id, &WalkSpeed), With<Monkey>>
) {
    for (mut vel, id, speed) in query.iter_mut() {
        if id.0 != 0 {
            continue;
        }
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
        x *= PLAYER_MAX_SPEED;
        y *= PLAYER_MAX_SPEED;
        let target_vel = Velocity{x,y};
        let new_vel = _interpolate_velocity(vel.clone(), target_vel, PLAYER_ACCEL);
        println!("start vel: {:?}, target vel {:?}, new_vel {:?}", vel, target_vel, new_vel);
        *vel = new_vel;
    }
}

fn handle_movement (
    time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Monkey>>
) {
    let (mut player_transform, vel) = query.single_mut();
    player_transform.translation.x += vel.x * time.delta_seconds();
    player_transform.translation.y += vel.y * time.delta_seconds();
}

fn _interpolate_velocity (start_vel: Velocity, target_vel: Velocity, acceleration: f32) -> Velocity {
    Velocity{x: _interpolate_dim(start_vel.x, target_vel.x, acceleration), y: _interpolate_dim(start_vel.y, target_vel.y, acceleration)}
}
fn _interpolate_dim(start: f32, target: f32, acceleration: f32) -> f32 {
    let diff = target - start;
    if diff > 0.0 {
        return start + f32::min(acceleration, diff);
    }
    start - f32::min(acceleration, f32::abs(diff))
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(MyTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .insert_resource(HeartbeatTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(INITIAL_ENEMY_SPAWN_RATE, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate, (
                heartbeat, 
                ping_all_monkeys, 
                handle_input,
                handle_movement,
                bevy::window::close_on_esc
            ).chain()
        )
        .run();
}