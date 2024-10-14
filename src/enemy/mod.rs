use self::path::{Path, PathMaker};
use crate::components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity};
use crate::{
	EnemyCount, GameTextures, WinSize, STAR_FIGHTER_LASER_SIZE, STAR_FIGHTER_MAX, STAR_FIGHTER_SIZE, SPRITE_SCALE,
};

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::{thread_rng, Rng};
use std::{f32::consts::PI, time::Duration};

mod path;

pub struct StarFighterPlugin;

impl Plugin for StarFighterPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(PathMaker::default())
			.add_systems(Update, star_fighter_spawn_system.run_if(on_timer(Duration::from_secs(1))))
			.add_systems(Update, star_fighter_fire_system.run_if(star_fighter_fire_criteria))
			.add_systems(Update, star_fighter_path_system);
	}
}

fn star_fighter_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	mut enemy_count: ResMut<EnemyCount>,
	mut formation_maker: ResMut<PathMaker>,
	win_size: Res<WinSize>,
) {
	if enemy_count.0 < STAR_FIGHTER_MAX {
		// get formation and start x/y
		let formation = formation_maker.make(&win_size);
		let (x, y) = formation.start;

		commands
			.spawn(SpriteBundle {
				texture: game_textures.enemy.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Enemy)
			.insert(formation)
			.insert(SpriteSize::from(STAR_FIGHTER_SIZE));

		enemy_count.0 += 1;
	}
}

fn star_fighter_fire_criteria() -> bool {
	thread_rng().gen_bool(1. / 60.)
}

fn star_fighter_fire_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	enemy_query: Query<&Transform, With<Enemy>>,
) {
	for &tf in enemy_query.iter() {
		let (x, y) = (tf.translation.x, tf.translation.y);
		// spawn enemy laser sprite
		commands
			.spawn(SpriteBundle {
				texture: game_textures.enemy_laser.clone(),
				transform: Transform {
					translation: Vec3::new(x, y - 15., 0.),
					rotation: Quat::from_rotation_x(PI),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
				},
				..Default::default()
			})
			.insert(Laser)
			.insert(SpriteSize::from(STAR_FIGHTER_LASER_SIZE))
			.insert(FromEnemy)
			.insert(Movable { auto_despawn: true })
			.insert(Velocity { x: 0., y: -1. });
	}
}

fn star_fighter_path_system(
	time: Res<Time>,
	mut query: Query<(&mut Transform, &mut Path), With<Enemy>>,
) {
	let delta = time.delta_seconds();

	for (mut transform, mut formation) in &mut query {
		// current position
		let (x_org, y_org) = (transform.translation.x, transform.translation.y);

		// maximum distance
		let max_distance = delta * formation.speed;

		// for counter clockwise 1, else -1
		let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. };
		let (x_pivot, y_pivot) = formation.pivot;
		let (x_radius, y_radius) = formation.radius;

		// get next angle (based on time for now)
		let angle = formation.angle
			+ dir * formation.speed * delta / (x_radius.min(y_radius) * PI / 2.);

		// evaluate target x/y
		let x_dst = x_radius * angle.cos() + x_pivot;
		let y_dst = y_radius * angle.sin() + y_pivot;

		// evaluate distance
		let dx = x_org - x_dst;
		let dy = y_org - y_dst;
		let distance = (dx * dx + dy * dy).sqrt();
		let distance_ratio = if distance == 0. { 0. } else { max_distance / distance };

		// evaluate final x/y
		let x = x_org - dx * distance_ratio;
		let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
		let y = y_org - dy * distance_ratio;
		let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

		// begin rotating the formation angle only when the sprite is on or near the ellipse.
		if distance < max_distance * formation.speed / 20. {
			formation.angle = angle;
		}

		let translation = &mut transform.translation;
		(translation.x, translation.y) = (x, y);
	}
}