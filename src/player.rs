use bevy::prelude::*;
use crate::{GameCamera, GameConfig};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    handle_shooting,
                    update_player_effects,
                    sync_camera_to_player,
                ),
            );
    }
}

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub is_shooting: bool,
    pub last_shot: f32,
    pub psychedelic_charge: f32,
    pub kill_count: u32,
    pub score: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            speed: 5.0,
            is_shooting: false,
            last_shot: 0.0,
            psychedelic_charge: 0.0,
            kill_count: 0,
            score: 0.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerBody;

fn setup_player(mut commands: Commands) {
    // Create an invisible player body for physics
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        Player::default(),
        PlayerBody,
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<GameCamera>)>,
    camera_query: Query<&Transform, (With<GameCamera>, Without<Player>)>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    if let (Ok(mut player_transform), Ok(camera_transform)) = 
        (player_query.get_single_mut(), camera_query.get_single()) 
    {
        let mut direction = Vec3::ZERO;
        let forward = camera_transform.forward();
        let right = camera_transform.right();

        // WASD movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += *forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= *forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= *right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += *right;
        }

        // Sprinting
        let speed_multiplier = if keyboard_input.pressed(KeyCode::ShiftLeft) {
            1.5
        } else {
            1.0
        };

        // Normalize and apply movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
            player_transform.translation += direction 
                * config.movement_speed 
                * speed_multiplier 
                * time.delta_seconds();
        }

        // Update camera to follow player
        // We'll handle this in a separate system to avoid borrowing conflicts
    }
}

fn handle_shooting(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let current_time = time.elapsed_seconds();
        
        if mouse_input.pressed(MouseButton::Left) {
            // Rapid fire - 10 shots per second
            if current_time - player.last_shot > 0.1 {
                player.is_shooting = true;
                player.last_shot = current_time;
                
                // Increase psychedelic charge when shooting
                player.psychedelic_charge = (player.psychedelic_charge + 0.1).min(1.0);
            }
        } else {
            player.is_shooting = false;
            // Slowly decrease psychedelic charge when not shooting
            player.psychedelic_charge = (player.psychedelic_charge - time.delta_seconds() * 0.5).max(0.0);
        }
    }
}

fn update_player_effects(
    mut player_query: Query<&mut Player>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    mut config: ResMut<GameConfig>,
    time: Res<Time>,
) {
    if let (Ok(player), Ok(mut camera_transform)) = 
        (player_query.get_single_mut(), camera_query.get_single_mut()) 
    {
        // Update psychedelic intensity based on player state
        config.psychedelic_intensity = 0.5 + player.psychedelic_charge * 0.5;
        
        // Screen shake when shooting
        if player.is_shooting {
            let shake_strength = 0.01;
            let shake_x = (time.elapsed_seconds() * 50.0).sin() * shake_strength;
            let shake_y = (time.elapsed_seconds() * 60.0).cos() * shake_strength;
            
            camera_transform.translation += Vec3::new(shake_x, shake_y, 0.0);
        }
    }
}

// Sync camera position with player position
pub fn sync_camera_to_player(
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) = 
        (player_query.get_single(), camera_query.get_single_mut()) 
    {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.z = player_transform.translation.z;
        // Keep camera at eye height (1.8 units above player)
        camera_transform.translation.y = player_transform.translation.y + 1.8;
    }
} 