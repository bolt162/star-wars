use crate::components::{FromFalcon, Laser, Movable, Falcon, SpriteSize, Velocity};
use crate::{
	GameTextures, PlayerState, WinSize, FALCON_LASER_SIZE, FALCON_RESPAWN_DELAY, FALCON_SIZE,
	SPRITE_SCALE,
};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

pub struct FalconPlugin;

impl Plugin for FalconPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(PlayerState::default())
			.add_systems(
				Update,
				falcon_spawn_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
			)
			.add_systems(Update, falcon_input_system)
			.add_systems(Update, falcon_laser_system);
	}
}

fn falcon_spawn_system(
	mut commands: Commands,
	mut player_state: ResMut<PlayerState>,
	time: Res<Time>,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
) {
	let now = time.elapsed_seconds_f64();
	let last_shot = player_state.last_shot;

	if !player_state.on && (last_shot == -1. || now > last_shot + FALCON_RESPAWN_DELAY) {
		// add player
		let bottom = -win_size.h / 2.;
		commands
			.spawn(SpriteBundle {
				texture: game_textures.player.clone(),
				transform: Transform {
					translation: Vec3::new(
						0.,
						bottom + FALCON_SIZE.1 / 2. * SPRITE_SCALE + 5.,
						10.,
					),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Falcon)
			.insert(SpriteSize::from(FALCON_SIZE))
			.insert(Movable { auto_despawn: false })
			.insert(Velocity { x: 0., y: 0. });

		player_state.spawned();
	}
}

fn falcon_laser_system(
	mut commands: Commands,
	kb: Res<ButtonInput<KeyCode>>,
	game_textures: Res<GameTextures>,
	query: Query<&Transform, With<Falcon>>,
) {
	if let Ok(player_tf) = query.get_single() {
		if kb.just_pressed(KeyCode::Space) {
			let (x, y) = (player_tf.translation.x, player_tf.translation.y);
			let x_offset = FALCON_SIZE.0 / 2. * SPRITE_SCALE - 5.;

			let mut spawn_laser = |x_offset: f32| {
				commands
					.spawn(SpriteBundle {
						texture: game_textures.player_laser.clone(),
						transform: Transform {
							translation: Vec3::new(x + x_offset, y + 15., 0.),
							scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
							..Default::default()
						},
						..Default::default()
					})
					.insert(Laser)
					.insert(FromFalcon)
					.insert(SpriteSize::from(FALCON_LASER_SIZE))
					.insert(Movable { auto_despawn: true })
					.insert(Velocity { x: 0., y: 1. });
			};

			spawn_laser(x_offset);
			spawn_laser(-x_offset);
		}
	}
}

fn falcon_input_system(
	kb: Res<ButtonInput<KeyCode>>,
	mut query: Query<&mut Velocity, With<Falcon>>,
) {
	if let Ok(mut velocity) = query.get_single_mut() {
		velocity.x = if kb.pressed(KeyCode::ArrowLeft) {
			-1.
		} else if kb.pressed(KeyCode::ArrowRight) {
			1.
		} else {
			0.
		}
	}
}