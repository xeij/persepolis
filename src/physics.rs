use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_basic_physics,
            check_arena_bounds,
        ));
    }
}

#[derive(Component)]
pub struct RigidBody {
    pub velocity: Vec3,
    pub mass: f32,
    pub friction: f32,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            mass: 1.0,
            friction: 0.95,
        }
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            radius: 0.5,
        }
    }
}

fn apply_basic_physics(
    mut query: Query<(&mut Transform, &mut RigidBody)>,
    time: Res<Time>,
) {
    for (mut transform, mut rigidbody) in query.iter_mut() {
        // Apply velocity
        transform.translation += rigidbody.velocity * time.delta_seconds();
        
        // Apply friction
        let friction = rigidbody.friction;
        rigidbody.velocity *= friction;
        
        // Zero out very small velocities
        if rigidbody.velocity.length() < 0.01 {
            rigidbody.velocity = Vec3::ZERO;
        }
    }
}

fn check_arena_bounds(
    mut query: Query<&mut Transform, With<crate::Player>>,
) {
    let arena_size = 24.0; // Slightly smaller than walls
    
    for mut transform in query.iter_mut() {
        // Clamp player position within arena bounds
        transform.translation.x = transform.translation.x.clamp(-arena_size, arena_size);
        transform.translation.z = transform.translation.z.clamp(-arena_size, arena_size);
        
        // Keep player above ground
        transform.translation.y = transform.translation.y.max(0.0);
    }
} 