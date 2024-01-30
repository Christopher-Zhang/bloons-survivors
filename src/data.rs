use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Alive,
    Paused,
    Dead,
}
pub const BACKGROUND_COLOR: Color = Color::rgb(0.3, 0.6, 0.3);
pub const DEFAULT_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const DEFAULT_SCALE: Vec3 = Vec3::new(3.0, 3.0, 1.0);
pub const PLAYER_MAX_SPEED: f32 = 500.0;
pub const PLAYER_ACCEL: f32 = 200.0;
pub const ENEMY_MAX_SPEED: f32 = 250.0;
pub const ENEMY_ACCEL: f32 = 50.0;
pub const ENEMY_SPAWN_DIST: f32 = 2000.0;
pub const ENEMY_SPAWN_WIGGLE: f32 = 300.0;
pub const INITIAL_ENEMY_SPAWN_RATE: f32 = 1.0;
pub const DELTA_ENEMY_SPAWN_RATE: f32 = 0.1;
pub const MINIMUM_ENEMY_SPAWN_RATE: f32 = 0.2;
pub const ZERO_VELOCITY: Velocity = Velocity { x: 0.0, y: 0.0 , last_dir: Vec3::X};
pub const PLAYER_STARTING_HEALTH: f32 = 10.0;
pub const SPRITE_WIDTH: f32 = 32.0;
pub const SPRITE_HEIGHT: f32 = 32.0;
pub const DEFAULT_HITBOX_WIDTH: f32 = 16.0;
pub const DEFAULT_HITBOX_HEIGHT: f32 = 16.0;
pub const MOB_HITBOX_LENGTH: f32 = 30.;
pub const PROJECTILE_HITBOX_LENGTH: f32 = 30.;
pub const HEALTHBAR_COLOR: Color = Color::CRIMSON;
pub const HEALTHBAR_HEIGHT: f32 = 10.0;
pub const DEFAULT_DIR: Vec3 = Vec3::X;
pub enum Trajectory {
    Line,
}
pub struct WeaponPath {
    pub starting_angles: Vec<f32>,
    pub starting_distances: Vec<f32>,
    pub trajectory: Trajectory,
}
impl WeaponPath {
    pub fn new(starting_angles: Vec<f32>, starting_distances: Vec<f32>, trajectory: Trajectory) -> WeaponPath {
        WeaponPath {
            starting_angles,
            starting_distances,
            trajectory
        }
    }
}
#[derive(Component)]
pub struct Weapon {
    pub sprite_path: String,
    pub base_damage: f32,
    pub damage_modifier: f32,
    pub pierce: i32,
    pub missile_life: i32,
    pub missile_speed: f32,
    pub weapon_path: WeaponPath,
    pub base_attack_speed: i32,
    pub frames_left: i32,
}
impl Weapon {
    pub fn new(
        sprite_path: String,
        base_damage: f32,
        damage_modifier: f32,
        pierce: i32,
        missile_life: i32,
        missile_speed: f32,
        weapon_path: WeaponPath,
        base_attack_speed: i32,
        frames_left: i32,
    ) -> Weapon {
        Weapon {
            sprite_path,
            base_damage,
            damage_modifier,
            pierce,
            missile_life,
            missile_speed,
            weapon_path,
            base_attack_speed,
            frames_left,
        }
    }
}

#[derive(Component)]
pub struct Projectile {
    pub pierce: i32,
    pub damage: f32,
    pub missile_life: i32,
    pub speed: f32,
    pub angle: f32
}

impl Projectile {
    pub fn new(pierce: i32, damage: f32, missile_life: i32, speed: f32, angle: f32) -> Projectile {
        Projectile {
            pierce,
            damage,
            missile_life,
            speed,
            angle
        }
    }
}

#[derive(Component)]
pub struct Healthbar;
#[derive(Component)]
pub struct Monkey;
// MaxHealth
// CurrentHealth
// MovementSpeed
#[derive(Component)]
pub struct Bloon;
// Damage
// MaxHealth
// CurrentHealth
// MovementSpeed
#[derive(Component, Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub last_dir: Vec3
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y, last_dir: Vec3::X }
    }
    pub fn from_vec3(v: &Vec3) -> Velocity {
        let mut last_dir = Vec3::normalize_or_zero(v.clone());
        if last_dir.x == 0.0 && last_dir.y == 0.0 {
            last_dir = Vec3::X;
        }
        Velocity { x: v.x, y: v.y, last_dir}
    }
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
    pub fn set(&mut self, x: f32, y: f32) -> Velocity {
        self.x = x;
        self.y = y;
        self.last_dir = Vec3::try_normalize(Vec3::new(x,y, 0.0)).unwrap_or(self.last_dir);
        *self
    }
    pub fn get_dir(x: f32, y: f32) -> Vec3 {
        Vec3::try_normalize(Vec3::new(x,y, 0.0)).unwrap_or(Vec3::X)
    }

}
#[derive(Component, Clone)]
pub struct MovementSpeed(pub f32);
#[derive(Component, Clone)]
pub struct CurrentHealth(pub f32);
#[derive(Component, Clone)]
pub struct MaxHealth(pub f32);
#[derive(Component, Clone)]
pub struct Damage(pub f32);
#[derive(Component, Clone)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
}
impl Hitbox {
    pub fn new(width: f32, height: f32) -> Hitbox {
        Hitbox {width, height}
    }
    pub fn both(length: f32) -> Hitbox {
        Hitbox{ width: length, height: length }
    }
}
#[derive(Component)]
pub struct CollisionFlags(Vec<usize>);
#[derive(Resource)]
pub struct MyTimer(pub Timer);
#[derive(Resource)]
pub struct HeartbeatTimer(pub Timer);
#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);
#[derive(Resource)]
pub struct EnemyLevelUpTimer(pub Timer);
