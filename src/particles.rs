use bevy::prelude::*;
use rand::Rng;
use crate::zombies::ZombieType;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_particles,
                update_expanding_rings,
                update_screen_flash,
                cleanup_particles,
                cleanup_expanding_rings,
                cleanup_screen_flash,
            ),
        );
    }
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub color_shift_speed: f32,
    pub gravity: f32,
    pub particle_type: ParticleType,
}

#[derive(Clone)]
pub enum ParticleType {
    Explosion,
    Spark,
    Glow,
    Trail,
}

#[derive(Component)]
pub struct ExpandingRing {
    pub current_radius: f32,
    pub max_radius: f32,
    pub expansion_speed: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color_phase: f32,
}

#[derive(Component)]
pub struct ScreenFlash {
    pub intensity: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub flash_color: Color,
}

#[derive(Component)]
pub struct DeathEffect;

pub fn spawn_death_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    zombie_type: &ZombieType,
) {
    let mut rng = rand::thread_rng();
    
    // Spawn different effects based on zombie type
    match zombie_type {
        ZombieType::Basic => spawn_basic_death_effect(commands, meshes, materials, position),
        ZombieType::Fast => spawn_fast_death_effect(commands, meshes, materials, position),
        ZombieType::Heavy => spawn_heavy_death_effect(commands, meshes, materials, position),
        ZombieType::Exploder => spawn_exploder_death_effect(commands, meshes, materials, position),
    }
    
    // Always spawn expanding ring effect
    spawn_expanding_ring(commands, meshes, materials, position);
    
    // Always spawn screen flash
    spawn_screen_flash(commands, zombie_type);
    
    // Spawn rainbow particle explosion
    spawn_rainbow_explosion(commands, meshes, materials, position, &mut rng);
}

fn spawn_basic_death_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let mut rng = rand::thread_rng();
    
    // Spawn 15-25 particles
    let particle_count = rng.gen_range(15..25);
    for _ in 0..particle_count {
        let velocity = Vec3::new(
            rng.gen_range(-8.0..8.0),
            rng.gen_range(2.0..12.0),
            rng.gen_range(-8.0..8.0),
        );
        
        spawn_particle(
            commands,
            meshes,
            materials,
            position,
            velocity,
            ParticleType::Explosion,
            rng.gen_range(0.1..0.3),
            rng.gen_range(1.0..2.5),
        );
    }
}

fn spawn_fast_death_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let mut rng = rand::thread_rng();
    
    // Spawn many small, fast particles
    let particle_count = rng.gen_range(30..45);
    for _ in 0..particle_count {
        let velocity = Vec3::new(
            rng.gen_range(-15.0..15.0),
            rng.gen_range(1.0..8.0),
            rng.gen_range(-15.0..15.0),
        );
        
        spawn_particle(
            commands,
            meshes,
            materials,
            position,
            velocity,
            ParticleType::Spark,
            rng.gen_range(0.05..0.15),
            rng.gen_range(0.8..1.5),
        );
    }
}

fn spawn_heavy_death_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let mut rng = rand::thread_rng();
    
    // Spawn fewer, larger particles
    let particle_count = rng.gen_range(8..15);
    for _ in 0..particle_count {
        let velocity = Vec3::new(
            rng.gen_range(-5.0..5.0),
            rng.gen_range(3.0..10.0),
            rng.gen_range(-5.0..5.0),
        );
        
        spawn_particle(
            commands,
            meshes,
            materials,
            position,
            velocity,
            ParticleType::Glow,
            rng.gen_range(0.3..0.8),
            rng.gen_range(2.0..4.0),
        );
    }
}

fn spawn_exploder_death_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let mut rng = rand::thread_rng();
    
    // Spawn intense explosion with many particles
    let particle_count = rng.gen_range(50..80);
    for _ in 0..particle_count {
        let velocity = Vec3::new(
            rng.gen_range(-20.0..20.0),
            rng.gen_range(5.0..15.0),
            rng.gen_range(-20.0..20.0),
        );
        
        spawn_particle(
            commands,
            meshes,
            materials,
            position,
            velocity,
            ParticleType::Explosion,
            rng.gen_range(0.2..0.5),
            rng.gen_range(1.5..3.0),
        );
    }
}

fn spawn_rainbow_explosion(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    rng: &mut rand::prelude::ThreadRng,
) {
    // Create a circular burst of rainbow particles
    let particle_count = 24; // 360/15 = 24 particles in a circle
    for i in 0..particle_count {
        let angle = (i as f32 / particle_count as f32) * std::f32::consts::TAU;
        let speed = rng.gen_range(8.0..12.0);
        
        let velocity = Vec3::new(
            angle.cos() * speed,
            rng.gen_range(2.0..6.0),
            angle.sin() * speed,
        );
        
        spawn_particle(
            commands,
            meshes,
            materials,
            position,
            velocity,
            ParticleType::Trail,
            rng.gen_range(0.15..0.25),
            rng.gen_range(2.0..3.5),
        );
    }
}

fn spawn_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    velocity: Vec3,
    particle_type: ParticleType,
    size: f32,
    lifetime: f32,
) {
    let mesh = meshes.add(Mesh::from(Sphere::new(size)));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        emissive: Color::srgb(2.0, 1.0, 0.5).into(),
        ..default()
    });
    
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_translation(position),
            ..default()
        },
        Particle {
            velocity,
            lifetime: 0.0,
            max_lifetime: lifetime,
            size,
            color_shift_speed: rand::thread_rng().gen_range(2.0..5.0),
            gravity: -9.8,
            particle_type,
        },
        DeathEffect,
    ));
}

fn spawn_expanding_ring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let ring_mesh = meshes.add(Mesh::from(Torus::new(0.1, 0.5)));
    let ring_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.7),
        emissive: Color::srgb(2.0, 1.0, 2.0).into(),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    
    commands.spawn((
        PbrBundle {
            mesh: ring_mesh,
            material: ring_material,
            transform: Transform::from_translation(position),
            ..default()
        },
        ExpandingRing {
            current_radius: 0.5,
            max_radius: 8.0,
            expansion_speed: 12.0,
            lifetime: 0.0,
            max_lifetime: 1.5,
            color_phase: 0.0,
        },
        DeathEffect,
    ));
}

fn spawn_screen_flash(commands: &mut Commands, zombie_type: &ZombieType) {
    let (intensity, color, duration) = match zombie_type {
        ZombieType::Basic => (0.3, Color::srgb(1.0, 0.5, 0.5), 0.2),
        ZombieType::Fast => (0.4, Color::srgb(0.5, 1.0, 0.5), 0.15),
        ZombieType::Heavy => (0.6, Color::srgb(0.5, 0.5, 1.0), 0.4),
        ZombieType::Exploder => (0.8, Color::srgb(1.0, 0.8, 0.2), 0.3),
    };
    
    commands.spawn((
        ScreenFlash {
            intensity,
            lifetime: 0.0,
            max_lifetime: duration,
            flash_color: color,
        },
        DeathEffect,
    ));
}

fn update_particles(
    mut particle_query: Query<(&mut Transform, &mut Particle, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (mut transform, mut particle, material_handle) in particle_query.iter_mut() {
        // Update lifetime
        particle.lifetime += time.delta_seconds();
        
        // Apply physics
        particle.velocity.y += particle.gravity * time.delta_seconds();
        transform.translation += particle.velocity * time.delta_seconds();
        
        // Update scale based on lifetime
        let life_ratio = particle.lifetime / particle.max_lifetime;
        let scale_factor = 1.0 - (life_ratio * life_ratio); // Quadratic falloff
        transform.scale = Vec3::splat(scale_factor);
        
        // Update rainbow colors
        if let Some(material) = materials.get_mut(&*material_handle) {
            let color_phase = particle.lifetime * particle.color_shift_speed;
            let rainbow_color = get_rainbow_color(color_phase);
            let intensity = (1.0 - life_ratio) * 3.0; // Fade intensity
            
            material.emissive = Color::srgb(
                rainbow_color.to_srgba().red * intensity,
                rainbow_color.to_srgba().green * intensity,
                rainbow_color.to_srgba().blue * intensity,
            ).into();
            material.base_color = rainbow_color;
        }
    }
}

fn update_expanding_rings(
    mut ring_query: Query<(&mut Transform, &mut ExpandingRing, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (mut transform, mut ring, material_handle) in ring_query.iter_mut() {
        // Update lifetime and radius
        ring.lifetime += time.delta_seconds();
        ring.current_radius += ring.expansion_speed * time.delta_seconds();
        ring.color_phase += time.delta_seconds() * 4.0;
        
        // Update scale
        let scale = ring.current_radius / 0.5; // Initial radius was 0.5
        transform.scale = Vec3::new(scale, 1.0, scale);
        
        // Update color and transparency
        if let Some(material) = materials.get_mut(&*material_handle) {
            let life_ratio = ring.lifetime / ring.max_lifetime;
            let alpha = 1.0 - life_ratio;
            let rainbow_color = get_rainbow_color(ring.color_phase);
            
            let srgba = rainbow_color.to_srgba();
            material.base_color = Color::srgba(srgba.red, srgba.green, srgba.blue, alpha * 0.7);
            material.emissive = Color::srgb(
                srgba.red * (1.0 - life_ratio) * 2.0,
                srgba.green * (1.0 - life_ratio) * 2.0,
                srgba.blue * (1.0 - life_ratio) * 2.0,
            ).into();
        }
    }
}

fn update_screen_flash(
    mut flash_query: Query<&mut ScreenFlash>,
    time: Res<Time>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let mut total_flash_intensity = 0.0;
    let mut flash_color = Color::srgb(0.0, 0.0, 0.0);
    
    for mut flash in flash_query.iter_mut() {
        flash.lifetime += time.delta_seconds();
        
        let life_ratio = flash.lifetime / flash.max_lifetime;
        let intensity = flash.intensity * (1.0 - life_ratio * life_ratio); // Quadratic falloff
        
        total_flash_intensity += intensity;
        let flash_srgba = flash_color.to_srgba();
        let flash_flash_srgba = flash.flash_color.to_srgba();
        flash_color = Color::srgb(
            flash_srgba.red + flash_flash_srgba.red * intensity,
            flash_srgba.green + flash_flash_srgba.green * intensity,
            flash_srgba.blue + flash_flash_srgba.blue * intensity,
        );
    }
    
    // Apply flash to ambient lighting
    if total_flash_intensity > 0.0 {
        ambient_light.color = flash_color;
        ambient_light.brightness = 0.1 + total_flash_intensity;
    } else {
        // Reset to normal ambient
        ambient_light.color = Color::srgb(0.2, 0.1, 0.4);
        ambient_light.brightness = 0.1;
    }
}

fn cleanup_particles(
    mut commands: Commands,
    particle_query: Query<(Entity, &Particle)>,
) {
    for (entity, particle) in particle_query.iter() {
        if particle.lifetime >= particle.max_lifetime {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_expanding_rings(
    mut commands: Commands,
    ring_query: Query<(Entity, &ExpandingRing)>,
) {
    for (entity, ring) in ring_query.iter() {
        if ring.lifetime >= ring.max_lifetime || ring.current_radius >= ring.max_radius {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_screen_flash(
    mut commands: Commands,
    flash_query: Query<(Entity, &ScreenFlash)>,
) {
    for (entity, flash) in flash_query.iter() {
        if flash.lifetime >= flash.max_lifetime {
            commands.entity(entity).despawn();
        }
    }
}

fn get_rainbow_color(phase: f32) -> Color {
    let r = (phase.sin() + 1.0) * 0.5;
    let g = ((phase + 2.094).sin() + 1.0) * 0.5; // 120 degrees phase shift
    let b = ((phase + 4.188).sin() + 1.0) * 0.5; // 240 degrees phase shift
    Color::srgb(r, g, b)
} 