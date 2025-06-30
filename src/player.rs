use bevy::prelude::*;
use crate::{GameCamera, GameConfig, GameState, physics::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    handle_shooting,
                    update_player_effects,
                    sync_camera_to_player,
                ).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), cleanup_player)
            .add_systems(OnEnter(GameState::InGame), reset_player_on_restart);
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
    pub acceleration: f32,
    pub air_control: f32,
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
            acceleration: 20.0,
            air_control: 0.3,
        }
    }
}

#[derive(Component)]
pub struct PlayerBody;

fn setup_player(mut commands: Commands) {
    // Create player with physics components
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)), // Start slightly above ground
        Player::default(),
        PlayerBody,
        RigidBody {
            velocity: Vec3::ZERO,
            mass: 70.0, // 70kg player
            friction: 0.8,
            restitution: 0.0, // No bouncing for player
            drag: 0.95,
            is_kinematic: false,
        },
        Collider {
            radius: 0.5,
            collision_layer: CollisionLayer::Player,
            collision_mask: CollisionLayer::Zombie.mask() | CollisionLayer::Environment.mask(),
        },
        GroundDetector::default(),
        Jumper {
            can_jump: true,
            jump_count: 0,
            max_jumps: 1, // Single jump for now
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut RigidBody, &mut Player, &GroundDetector), (With<Player>, Without<GameCamera>)>,
    camera_query: Query<&Transform, (With<GameCamera>, Without<Player>)>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    if let (Ok((_player_transform, mut rigidbody, _player, ground_detector)), Ok(camera_transform)) = 
        (player_query.get_single_mut(), camera_query.get_single()) 
    {
        let mut direction = Vec3::ZERO;
        let forward = camera_transform.forward();
        let right = camera_transform.right();

        // Get movement input
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

        // Flatten direction to horizontal plane
        direction.y = 0.0;

        // Sprinting
        let speed_multiplier = if keyboard_input.pressed(KeyCode::ShiftLeft) {
            1.5
        } else {
            1.0
        };

        // Apply physics-based movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
            
            // Calculate target velocity
            let target_velocity = direction * config.movement_speed * speed_multiplier;
            
            // Apply different acceleration based on ground state
            let acceleration = if ground_detector.is_grounded {
                player.acceleration
            } else {
                player.acceleration * player.air_control // Reduced air control
            };
            
            // Smoothly accelerate towards target velocity
            let velocity_change = target_velocity - Vec3::new(rigidbody.velocity.x, 0.0, rigidbody.velocity.z);
            let acceleration_force = velocity_change * acceleration * time.delta_seconds();
            
            // Apply force to horizontal movement only
            rigidbody.velocity.x += acceleration_force.x;
            rigidbody.velocity.z += acceleration_force.z;
            
            // Clamp horizontal velocity to max speed
            let horizontal_velocity = Vec3::new(rigidbody.velocity.x, 0.0, rigidbody.velocity.z);
            let max_speed = config.movement_speed * speed_multiplier;
            
            if horizontal_velocity.length() > max_speed {
                let clamped = horizontal_velocity.normalize() * max_speed;
                rigidbody.velocity.x = clamped.x;
                rigidbody.velocity.z = clamped.z;
            }
        } else if ground_detector.is_grounded {
            // Apply stopping force when no input
            let stopping_force = 0.9;
            rigidbody.velocity.x *= stopping_force;
            rigidbody.velocity.z *= stopping_force;
        }
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

fn cleanup_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn reset_player_on_restart(mut player_query: Query<&mut Player>) {
    for mut player in player_query.iter_mut() {
        *player = Player::default();
    }
} 