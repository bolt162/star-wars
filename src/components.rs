use bevy::math::{Vec2, Vec3};
use bevy::prelude::Component;
use bevy::time::{Timer, TimerMode};

// region:    --- Common Components
#[derive(Component)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

#[derive(Component)]
pub struct Movable {
	pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
	fn from(val: (f32, f32)) -> Self {
		SpriteSize(Vec2::new(val.0, val.1))
	}
}

// endregion: --- Common Components

// region:    --- Player Components
#[derive(Component)]
pub struct Falcon;

#[derive(Component)]
pub struct FromFalcon;
// endregion: --- Player Components

#[derive(Component)]
pub struct Score(pub u32);

// region:    --- Enemy Components
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;
// endregion: --- Enemy Components

// region:    --- Explosion Components
#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
	fn default() -> Self {
		Self(Timer::from_seconds(0.05, TimerMode::Repeating))
	}
}
// endregion: --- Explosion Components