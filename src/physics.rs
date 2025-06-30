use bevy::prelude::*;
use crate::{GameState, Player, zombies::Zombie};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsSettings>()
            .add_systems(Update, (
                apply_gravity,
                apply_physics_movement,
                handle_jumping,
                player_zombie_collision,
                zombie_zombie_collision,
                check_arena_bounds,
                apply_ground_detection,
                apply_friction,
                update_collision_events,
            ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Resource)]
pub struct PhysicsSettings {
    pub gravity: f32,
    pub ground_level: f32,
    pub jump_force: f32,
    pub player_damage_cooldown: f32,
    pub collision_damping: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            gravity: -30.0,
            ground_level: 0.0,
            jump_force: 15.0,
            player_damage_cooldown: 1.0, // 1 second between damage
            collision_damping: 0.5,
        }
    }
}

#[derive(Component)]
pub struct RigidBody {
    pub velocity: Vec3,
    pub mass: f32,
    pub friction: f32,
    pub restitution: f32, // Bounciness
    pub drag: f32,
    pub is_kinematic: bool, // If true, not affected by forces
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            mass: 1.0,
            friction: 0.8,
            restitution: 0.1,
            drag: 0.98,
            is_kinematic: false,
        }
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub collision_layer: CollisionLayer,
    pub collision_mask: u32, // Which layers this collides with
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            radius: 0.5,
            collision_layer: CollisionLayer::Default,
            collision_mask: 0b11111111, // Collides with all layers by default
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CollisionLayer {
    Default = 0,
    Player = 1,
    Zombie = 2,
    Bullet = 3,
    Environment = 4,
}

impl CollisionLayer {
    pub fn mask(&self) -> u32 {
        1 << (*self as u32)
    }
}

#[derive(Component)]
pub struct GroundDetector {
    pub is_grounded: bool,
    pub ground_distance: f32,
}

impl Default for GroundDetector {
    fn default() -> Self {
        Self {
            is_grounded: false,
            ground_distance: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Jumper {
    pub can_jump: bool,
    pub jump_count: u32,
    pub max_jumps: u32, // For multi-jump
}

impl Default for Jumper {
    fn default() -> Self {
        Self {
            can_jump: true,
            jump_count: 0,
            max_jumps: 1, // Single jump by default
        }
    }
}

#[derive(Component)]
pub struct CollisionDamage {
    pub damage: f32,
    pub last_damage_time: f32,
}

impl Default for CollisionDamage {
    fn default() -> Self {
        Self {
            damage: 10.0,
            last_damage_time: 0.0,
        }
    }
}

// Physics Systems

fn apply_gravity(
    mut query: Query<(&mut RigidBody, &GroundDetector), Without<crate::Player>>,
    mut player_query: Query<(&mut RigidBody, &GroundDetector), With<crate::Player>>,
    settings: Res<PhysicsSettings>,
    time: Res<Time>,
) {
    let gravity_force = Vec3::new(0.0, settings.gravity, 0.0);
    let dt = time.delta_seconds();

    // Apply gravity to non-player entities
    for (mut rigidbody, ground_detector) in query.iter_mut() {
        if !rigidbody.is_kinematic && !ground_detector.is_grounded {
            rigidbody.velocity += gravity_force * dt;
        }
    }

    // Apply gravity to player
    for (mut rigidbody, ground_detector) in player_query.iter_mut() {
        if !rigidbody.is_kinematic && !ground_detector.is_grounded {
            rigidbody.velocity += gravity_force * dt;
        }
    }
}

fn apply_physics_movement(
    mut query: Query<(&mut Transform, &mut RigidBody)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    
    for (mut transform, mut rigidbody) in query.iter_mut() {
        if !rigidbody.is_kinematic {
            // Apply velocity to position
            transform.translation += rigidbody.velocity * dt;
            
            // Apply drag
            rigidbody.velocity *= rigidbody.drag;
            
            // Zero out very small velocities to prevent jitter
            if rigidbody.velocity.length() < 0.01 {
                rigidbody.velocity = Vec3::ZERO;
            }
        }
    }
}

fn handle_jumping(
    mut player_query: Query<(&mut RigidBody, &mut Jumper, &GroundDetector), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    settings: Res<PhysicsSettings>,
) {
    if let Ok((mut rigidbody, mut jumper, ground_detector)) = player_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            // Reset jump count when grounded
            if ground_detector.is_grounded {
                jumper.jump_count = 0;
            }
            
            // Check if we can jump
            if jumper.can_jump && jumper.jump_count < jumper.max_jumps {
                rigidbody.velocity.y = settings.jump_force;
                jumper.jump_count += 1;
                jumper.can_jump = false; // Prevent multiple jumps on single press
            }
        }
        
        if keyboard_input.just_released(KeyCode::Space) {
            jumper.can_jump = true;
        }
    }
}

fn apply_ground_detection(
    mut query: Query<(&Transform, &mut GroundDetector, &Collider, &mut Jumper)>,
    settings: Res<PhysicsSettings>,
) {
    for (transform, mut ground_detector, collider, mut jumper) in query.iter_mut() {
        let ground_check_distance = collider.radius + 0.1;
        let distance_to_ground = transform.translation.y - settings.ground_level;
        
        ground_detector.ground_distance = distance_to_ground;
        ground_detector.is_grounded = distance_to_ground <= ground_check_distance;
        
        // Reset jump count when grounded
        if ground_detector.is_grounded {
            jumper.jump_count = 0;
        }
    }
}

fn player_zombie_collision(
    mut player_query: Query<(&Transform, &Collider, &mut Player), (With<Player>, Without<Zombie>)>,
    zombie_query: Query<(&Transform, &Collider, &CollisionDamage), (With<Zombie>, Without<Player>)>,
    settings: Res<PhysicsSettings>,
    time: Res<Time>,
) {
    if let Ok((player_transform, player_collider, mut player)) = player_query.get_single_mut() {
        let current_time = time.elapsed_seconds();
        
        for (zombie_transform, zombie_collider, collision_damage) in zombie_query.iter() {
            let distance = player_transform.translation.distance(zombie_transform.translation);
            let collision_distance = player_collider.radius + zombie_collider.radius;
            
            if distance <= collision_distance {
                // Check damage cooldown to prevent rapid damage
                if current_time - collision_damage.last_damage_time > settings.player_damage_cooldown {
                    player.health -= collision_damage.damage;
                    println!("Player takes {} damage! Health: {}", collision_damage.damage, player.health);
                    
                    // Update damage time (we'd need to make collision_damage mutable for this)
                    // For now, we'll implement a different approach using a resource or event
                }
            }
        }
    }
}

fn zombie_zombie_collision(
    mut zombie_query: Query<(&mut Transform, &mut RigidBody, &Collider), With<Zombie>>,
    settings: Res<PhysicsSettings>,
) {
    let mut combinations = zombie_query.iter_combinations_mut();
    
    while let Some([
        (mut transform_a, mut rigidbody_a, collider_a),
        (mut transform_b, mut rigidbody_b, collider_b)
    ]) = combinations.fetch_next() {
        let distance = transform_a.translation.distance(transform_b.translation);
        let collision_distance = collider_a.radius + collider_b.radius;
        
        if distance <= collision_distance && distance > 0.0 {
            // Calculate separation vector
            let separation = (transform_a.translation - transform_b.translation).normalize();
            let overlap = collision_distance - distance;
            
            // Apply separation to prevent stacking
            let separation_force = separation * overlap * 0.5;
            
            // Move zombies apart based on their masses
            let total_mass = rigidbody_a.mass + rigidbody_b.mass;
            let mass_ratio_a = rigidbody_b.mass / total_mass;
            let mass_ratio_b = rigidbody_a.mass / total_mass;
            
            transform_a.translation += separation_force * mass_ratio_a;
            transform_b.translation -= separation_force * mass_ratio_b;
            
            // Apply collision impulse for realistic physics
            let relative_velocity = rigidbody_a.velocity - rigidbody_b.velocity;
            let impulse_magnitude = relative_velocity.dot(separation) * settings.collision_damping;
            
            if impulse_magnitude > 0.0 {
                let impulse = separation * impulse_magnitude;
                
                rigidbody_a.velocity -= impulse * mass_ratio_a;
                rigidbody_b.velocity += impulse * mass_ratio_b;
            }
        }
    }
}

fn check_arena_bounds(
    mut query: Query<(&mut Transform, &mut RigidBody, &Collider)>,
) {
    let arena_size = 24.0;
    
    for (mut transform, mut rigidbody, collider) in query.iter_mut() {
        let effective_radius = collider.radius;
        
        // X bounds
        if transform.translation.x + effective_radius > arena_size {
            transform.translation.x = arena_size - effective_radius;
            rigidbody.velocity.x = -rigidbody.velocity.x * rigidbody.restitution;
        } else if transform.translation.x - effective_radius < -arena_size {
            transform.translation.x = -arena_size + effective_radius;
            rigidbody.velocity.x = -rigidbody.velocity.x * rigidbody.restitution;
        }
        
        // Z bounds
        if transform.translation.z + effective_radius > arena_size {
            transform.translation.z = arena_size - effective_radius;
            rigidbody.velocity.z = -rigidbody.velocity.z * rigidbody.restitution;
        } else if transform.translation.z - effective_radius < -arena_size {
            transform.translation.z = -arena_size + effective_radius;
            rigidbody.velocity.z = -rigidbody.velocity.z * rigidbody.restitution;
        }
        
        // Ground bounds
        if transform.translation.y - effective_radius < 0.0 {
            transform.translation.y = effective_radius;
            if rigidbody.velocity.y < 0.0 {
                rigidbody.velocity.y = -rigidbody.velocity.y * rigidbody.restitution;
            }
        }
    }
}

fn apply_friction(
    mut query: Query<(&mut RigidBody, &GroundDetector)>,
    time: Res<Time>,
) {
    for (mut rigidbody, ground_detector) in query.iter_mut() {
        if ground_detector.is_grounded {
            // Apply ground friction
            let friction_force = rigidbody.friction * time.delta_seconds();
            rigidbody.velocity.x *= 1.0 - friction_force;
            rigidbody.velocity.z *= 1.0 - friction_force;
        }
    }
}

fn update_collision_events(
    mut collision_damage_query: Query<&mut CollisionDamage>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds();
    for mut collision_damage in collision_damage_query.iter_mut() {
        // This system would typically handle collision event updates
        // For now, we just ensure the damage time is properly tracked
        if collision_damage.last_damage_time == 0.0 {
            collision_damage.last_damage_time = current_time;
        }
    }
} 