use bevy::prelude::*;
use rand::Rng;
use crate::{GameConfig, Player, GameState, physics::*};

pub struct ZombiePlugin;

impl Plugin for ZombiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_zombie_system)
            .add_systems(
                Update,
                (
                    spawn_zombies,
                    zombie_ai,
                    zombie_movement,
                    zombie_attack,
                    cleanup_dead_zombies,
                    update_zombie_effects,
                ).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), cleanup_all_zombies);
    }
}

#[derive(Component)]
pub struct Zombie {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub damage: f32,
    pub attack_range: f32,
    pub last_attack: f32,
    pub zombie_type: ZombieType,
    pub pulsation_phase: f32,
    pub color_shift: f32,
}

#[derive(Clone)]
pub enum ZombieType {
    Basic,
    Fast,
    Heavy,
    Exploder,
}

impl Default for Zombie {
    fn default() -> Self {
        Self {
            health: 30.0,
            max_health: 30.0,
            speed: 2.0,
            damage: 10.0,
            attack_range: 2.0,
            last_attack: 0.0,
            zombie_type: ZombieType::Basic,
            pulsation_phase: 0.0,
            color_shift: 0.0,
        }
    }
}

#[derive(Component)]
pub struct ZombieBody;

#[derive(Resource)]
pub struct ZombieSpawnTimer {
    pub timer: Timer,
    pub spawn_points: Vec<Vec3>,
    pub max_zombies: usize,
}

impl Default for ZombieSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            spawn_points: vec![
                Vec3::new(10.0, 0.0, 10.0),
                Vec3::new(-10.0, 0.0, 10.0),
                Vec3::new(10.0, 0.0, -10.0),
                Vec3::new(-10.0, 0.0, -10.0),
                Vec3::new(0.0, 0.0, 15.0),
                Vec3::new(0.0, 0.0, -15.0),
                Vec3::new(15.0, 0.0, 0.0),
                Vec3::new(-15.0, 0.0, 0.0),
            ],
            max_zombies: 20,
        }
    }
}

fn setup_zombie_system(mut commands: Commands) {
    commands.init_resource::<ZombieSpawnTimer>();
}

fn spawn_zombies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_timer: ResMut<ZombieSpawnTimer>,
    zombie_query: Query<&Zombie>,
    _config: Res<GameConfig>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
    
    if spawn_timer.timer.just_finished() && zombie_query.iter().count() < spawn_timer.max_zombies {
        let mut rng = rand::thread_rng();
        let spawn_point = spawn_timer.spawn_points[rng.gen_range(0..spawn_timer.spawn_points.len())];
        
        // Create different zombie types with psychedelic geometric shapes
        let zombie_type = match rng.gen_range(0..4) {
            0 => ZombieType::Basic,
            1 => ZombieType::Fast,
            2 => ZombieType::Heavy,
            _ => ZombieType::Exploder,
        };
        
        let (mesh, material, zombie) = match zombie_type {
            ZombieType::Basic => {
                let mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 2.0, 1.0)));
                let material = materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.0, 0.5),
                    emissive: Color::srgb(0.5, 0.0, 0.2).into(),
                    ..default()
                });
                (mesh, material, Zombie::default())
            },
            ZombieType::Fast => {
                let mesh = meshes.add(Mesh::from(Sphere::new(0.8)));
                let material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 1.0, 0.5),
                    emissive: Color::srgb(0.0, 0.5, 0.2).into(),
                    ..default()
                });
                let zombie = Zombie {
                    speed: 4.0,
                    health: 15.0,
                    max_health: 15.0,
                    zombie_type: ZombieType::Fast,
                    ..Zombie::default()
                };
                (mesh, material, zombie)
            },
            ZombieType::Heavy => {
                let mesh = meshes.add(Mesh::from(Cuboid::new(1.5, 2.5, 1.5)));
                let material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.5, 0.0, 1.0),
                    emissive: Color::srgb(0.2, 0.0, 0.5).into(),
                    ..default()
                });
                let zombie = Zombie {
                    speed: 1.0,
                    health: 60.0,
                    max_health: 60.0,
                    damage: 20.0,
                    zombie_type: ZombieType::Heavy,
                    ..Zombie::default()
                };
                (mesh, material, zombie)
            },
            ZombieType::Exploder => {
                let mesh = meshes.add(Mesh::from(Sphere::new(0.6)));
                let material = materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.5, 0.0),
                    emissive: Color::srgb(0.8, 0.3, 0.0).into(),
                    ..default()
                });
                let zombie = Zombie {
                    speed: 3.0,
                    health: 10.0,
                    max_health: 10.0,
                    damage: 50.0,
                    attack_range: 5.0,
                    zombie_type: ZombieType::Exploder,
                    ..Zombie::default()
                };
                (mesh, material, zombie)
            },
        };
        
        // Add physics components based on zombie type
        let (rigidbody, collider, collision_damage) = match zombie.zombie_type {
            ZombieType::Basic => (
                RigidBody {
                    velocity: Vec3::ZERO,
                    mass: 80.0,
                    friction: 0.7,
                    restitution: 0.1,
                    drag: 0.85,
                    is_kinematic: false,
                },
                Collider {
                    radius: 0.6,
                    collision_layer: CollisionLayer::Zombie,
                    collision_mask: CollisionLayer::Player.mask() | CollisionLayer::Zombie.mask() | CollisionLayer::Environment.mask(),
                },
                CollisionDamage {
                    damage: 10.0,
                    last_damage_time: 0.0,
                }
            ),
            ZombieType::Fast => (
                RigidBody {
                    velocity: Vec3::ZERO,
                    mass: 60.0,
                    friction: 0.6,
                    restitution: 0.2,
                    drag: 0.9,
                    is_kinematic: false,
                },
                Collider {
                    radius: 0.5,
                    collision_layer: CollisionLayer::Zombie,
                    collision_mask: CollisionLayer::Player.mask() | CollisionLayer::Zombie.mask() | CollisionLayer::Environment.mask(),
                },
                CollisionDamage {
                    damage: 5.0,
                    last_damage_time: 0.0,
                }
            ),
            ZombieType::Heavy => (
                RigidBody {
                    velocity: Vec3::ZERO,
                    mass: 120.0,
                    friction: 0.8,
                    restitution: 0.05,
                    drag: 0.8,
                    is_kinematic: false,
                },
                Collider {
                    radius: 0.8,
                    collision_layer: CollisionLayer::Zombie,
                    collision_mask: CollisionLayer::Player.mask() | CollisionLayer::Zombie.mask() | CollisionLayer::Environment.mask(),
                },
                CollisionDamage {
                    damage: 20.0,
                    last_damage_time: 0.0,
                }
            ),
            ZombieType::Exploder => (
                RigidBody {
                    velocity: Vec3::ZERO,
                    mass: 50.0,
                    friction: 0.5,
                    restitution: 0.3,
                    drag: 0.9,
                    is_kinematic: false,
                },
                Collider {
                    radius: 0.4,
                    collision_layer: CollisionLayer::Zombie,
                    collision_mask: CollisionLayer::Player.mask() | CollisionLayer::Zombie.mask() | CollisionLayer::Environment.mask(),
                },
                CollisionDamage {
                    damage: 50.0,
                    last_damage_time: 0.0,
                }
            ),
        };

        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(spawn_point),
                ..default()
            },
            zombie,
            ZombieBody,
            rigidbody,
            collider,
            collision_damage,
            GroundDetector::default(),
        ));
    }
}

fn zombie_ai(
    player_query: Query<&Transform, (With<Player>, Without<Zombie>)>,
    mut zombie_query: Query<&mut Transform, (With<Zombie>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut zombie_transform in zombie_query.iter_mut() {
            let direction = (player_transform.translation - zombie_transform.translation).normalize();
            zombie_transform.look_to(direction, Vec3::Y);
        }
    }
}

fn zombie_movement(
    player_query: Query<&Transform, (With<Player>, Without<Zombie>)>,
    mut zombie_query: Query<(&Transform, &mut RigidBody, &Zombie, &GroundDetector), Without<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (zombie_transform, mut rigidbody, zombie, ground_detector) in zombie_query.iter_mut() {
            let direction = player_transform.translation - zombie_transform.translation;
            let distance = direction.length();
            
            if distance > zombie.attack_range && distance > 0.0 {
                // Normalize direction and flatten to horizontal plane
                let mut move_direction = direction.normalize();
                move_direction.y = 0.0;
                
                // Calculate target velocity
                let target_velocity = move_direction * zombie.speed;
                
                // Apply acceleration towards target velocity
                let acceleration = if ground_detector.is_grounded { 15.0 } else { 5.0 };
                let velocity_change = target_velocity - Vec3::new(rigidbody.velocity.x, 0.0, rigidbody.velocity.z);
                let acceleration_force = velocity_change * acceleration * time.delta_seconds();
                
                // Apply force to horizontal movement only
                rigidbody.velocity.x += acceleration_force.x;
                rigidbody.velocity.z += acceleration_force.z;
                
                // Clamp horizontal velocity to max speed
                let horizontal_velocity = Vec3::new(rigidbody.velocity.x, 0.0, rigidbody.velocity.z);
                if horizontal_velocity.length() > zombie.speed {
                    let clamped = horizontal_velocity.normalize() * zombie.speed;
                    rigidbody.velocity.x = clamped.x;
                    rigidbody.velocity.z = clamped.z;
                }
            } else if ground_detector.is_grounded {
                // Apply stopping force when in attack range
                let stopping_force = 0.8;
                rigidbody.velocity.x *= stopping_force;
                rigidbody.velocity.z *= stopping_force;
            }
        }
    }
}

fn zombie_attack(
    player_query: Query<&Transform, (With<Player>, Without<Zombie>)>,
    mut zombie_query: Query<(&Transform, &mut Zombie), Without<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let current_time = time.elapsed_seconds();
        
        for (zombie_transform, mut zombie) in zombie_query.iter_mut() {
            let distance = zombie_transform.translation.distance(player_transform.translation);
            
            if distance <= zombie.attack_range && current_time - zombie.last_attack > 1.0 {
                zombie.last_attack = current_time;
                // TODO: Damage player system
                println!("Zombie attacks for {} damage!", zombie.damage);
            }
        }
    }
}

fn cleanup_dead_zombies(
    mut commands: Commands,
    zombie_query: Query<(Entity, &Zombie)>,
) {
    for (entity, zombie) in zombie_query.iter() {
        if zombie.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_zombie_effects(
    mut zombie_query: Query<(&mut Zombie, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    for (mut zombie, material_handle) in zombie_query.iter_mut() {
        zombie.pulsation_phase += time.delta_seconds() * 2.0;
        zombie.color_shift += time.delta_seconds() * config.psychedelic_intensity;
        
        if let Some(material) = materials.get_mut(&*material_handle) {
            let pulse = (zombie.pulsation_phase.sin() + 1.0) * 0.5;
            let color_shift = zombie.color_shift.sin();
            
            // Psychedelic color shifting
            match zombie.zombie_type {
                ZombieType::Basic => {
                    material.emissive = Color::srgb(
                        0.5 + pulse * 0.5,
                        color_shift.abs() * 0.3,
                        0.2 + color_shift * 0.2,
                    ).into();
                },
                ZombieType::Fast => {
                    material.emissive = Color::srgb(
                        color_shift.abs() * 0.3,
                        0.5 + pulse * 0.5,
                        0.2 + color_shift * 0.2,
                    ).into();
                },
                ZombieType::Heavy => {
                    material.emissive = Color::srgb(
                        0.2 + color_shift * 0.2,
                        color_shift.abs() * 0.3,
                        0.5 + pulse * 0.5,
                    ).into();
                },
                ZombieType::Exploder => {
                    material.emissive = Color::srgb(
                        0.8 + pulse * 0.2,
                        0.3 + color_shift * 0.2,
                        color_shift.abs() * 0.1,
                    ).into();
                },
            }
        }
    }
}

fn cleanup_all_zombies(mut commands: Commands, zombie_query: Query<Entity, With<Zombie>>) {
    for entity in zombie_query.iter() {
        commands.entity(entity).despawn();
    }
} 