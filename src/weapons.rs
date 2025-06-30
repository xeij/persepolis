use bevy::prelude::*;
use crate::{GameCamera, Player, Zombie};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_bullets,
                move_bullets,
                bullet_collision,
                cleanup_bullets,
                update_bullet_effects,
            ),
        );
    }
}

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub trail_intensity: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            damage: 25.0,
            speed: 50.0,
            lifetime: 0.0,
            max_lifetime: 3.0,
            trail_intensity: 1.0,
        }
    }
}

#[derive(Component)]
pub struct BulletTrail;

fn spawn_bullets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Player>,
    camera_query: Query<&Transform, With<GameCamera>>,
) {
    if let (Ok(player), Ok(camera_transform)) = 
        (player_query.get_single(), camera_query.get_single()) 
    {
        if player.is_shooting {
            let forward = camera_transform.forward();
            let bullet_spawn = camera_transform.translation + forward * 1.0;
            
            // Create psychedelic bullet
            let bullet_mesh = meshes.add(Mesh::from(Sphere::new(0.1)));
            let bullet_material = materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 0.0),
                emissive: Color::srgb(2.0, 2.0, 0.5).into(),
                ..default()
            });
            
            commands.spawn((
                PbrBundle {
                    mesh: bullet_mesh,
                    material: bullet_material,
                    transform: Transform::from_translation(bullet_spawn)
                        .looking_to(forward, Vec3::Y),
                    ..default()
                },
                Bullet::default(),
            ));
        }
    }
}

fn move_bullets(
    mut bullet_query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        let forward = transform.forward();
        transform.translation += forward * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<Zombie>)>,
    mut zombie_query: Query<(Entity, &Transform, &mut Zombie), Without<Bullet>>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            for (_zombie_entity, zombie_transform, mut zombie) in zombie_query.iter_mut() {
                let distance = bullet_transform.translation.distance(zombie_transform.translation);
                
                if distance < 1.0 { // Hit detection radius
                    // Damage zombie
                    zombie.health -= 25.0;
                    
                    // Increase player score and psychedelic charge
                    player.score += 10.0;
                    player.psychedelic_charge = (player.psychedelic_charge + 0.2).min(1.0);
                    
                    // If zombie dies, increase kill count
                    if zombie.health <= 0.0 {
                        player.kill_count += 1;
                        player.score += 50.0;
                        
                        // Spawn death effect
                        spawn_death_effect(&mut commands, zombie_transform.translation);
                    }
                    
                    // Remove bullet
                    commands.entity(bullet_entity).despawn();
                    break;
                }
            }
        }
    }
}

fn cleanup_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in bullet_query.iter_mut() {
        bullet.lifetime += time.delta_seconds();
        
        if bullet.lifetime >= bullet.max_lifetime {
            commands.entity(entity).despawn();
        }
    }
}

fn update_bullet_effects(
    mut bullet_query: Query<(&mut Handle<StandardMaterial>, &Bullet)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (material_handle, bullet) in bullet_query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            let phase = time.elapsed_seconds() * 10.0;
            let pulse = (phase.sin() + 1.0) * 0.5;
            
            // Psychedelic bullet colors with lifetime factor
            let lifetime_factor = 1.0 - (bullet.lifetime / bullet.max_lifetime);
            material.emissive = Color::srgb(
                (2.0 + pulse * 0.5) * lifetime_factor,
                (2.0 + (phase * 1.2).cos() * 0.5) * lifetime_factor,
                (0.5 + (phase * 0.8).sin() * 0.5) * lifetime_factor,
            ).into();
        }
    }
}

fn spawn_death_effect(_commands: &mut Commands, position: Vec3) {
    // TODO: Implement psychedelic death particle effects
    // This could include:
    // - Particle systems with rainbow colors
    // - Expanding rings of light
    // - Screen flash effects
    // - Sound effects
    println!("Death effect at position: {:?}", position);
} 