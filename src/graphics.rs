use bevy::prelude::*;
use crate::GameConfig;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_lighting, setup_environment))
            .add_systems(Update, (update_lighting_effects, update_fog_effects));
    }
}

#[derive(Component)]
pub struct PsychedelicLight {
    pub base_intensity: f32,
    pub color_shift_speed: f32,
    pub intensity_pulse_speed: f32,
    pub time_accumulator: f32,
}

impl Default for PsychedelicLight {
    fn default() -> Self {
        Self {
            base_intensity: 1000.0,
            color_shift_speed: 2.0,
            intensity_pulse_speed: 3.0,
            time_accumulator: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct GraphicsSettings {
    pub fog_color: Color,
    pub fog_density: f32,
    pub ambient_intensity: f32,
    pub chromatic_aberration: f32,
    pub distortion_strength: f32,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            fog_color: Color::srgb(0.1, 0.0, 0.2),
            fog_density: 0.02,
            ambient_intensity: 0.1,
            chromatic_aberration: 0.01,
            distortion_strength: 0.05,
        }
    }
}

fn setup_lighting(mut commands: Commands) {
    // Ambient lighting for dark, mysterious atmosphere
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.2, 0.1, 0.4),
        brightness: 0.1,
    });

    // Main psychedelic point lights
    let light_positions = vec![
        Vec3::new(5.0, 10.0, 5.0),
        Vec3::new(-5.0, 10.0, 5.0),
        Vec3::new(5.0, 10.0, -5.0),
        Vec3::new(-5.0, 10.0, -5.0),
        Vec3::new(0.0, 15.0, 0.0),
    ];

    for (i, position) in light_positions.iter().enumerate() {
        let base_color = match i {
            0 => Color::srgb(1.0, 0.0, 0.5),
            1 => Color::srgb(0.0, 1.0, 0.5),
            2 => Color::srgb(0.5, 0.0, 1.0),
            3 => Color::srgb(1.0, 0.5, 0.0),
            _ => Color::srgb(0.5, 1.0, 1.0),
        };

        commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    color: base_color,
                    intensity: 1000.0,
                    range: 20.0,
                    radius: 1.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(*position),
                ..default()
            },
            PsychedelicLight {
                color_shift_speed: 1.0 + i as f32 * 0.5,
                intensity_pulse_speed: 2.0 + i as f32 * 0.3,
                ..default()
            },
        ));
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.init_resource::<GraphicsSettings>();

    // Create a psychedelic arena floor
    let floor_mesh = meshes.add(Mesh::from(Plane3d::default().mesh().size(50.0, 50.0)));
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.2),
        emissive: Color::srgb(0.05, 0.0, 0.1).into(),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: floor_mesh,
        material: floor_material,
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });

    // Create psychedelic arena walls
    create_arena_walls(&mut commands, &mut meshes, &mut materials);
}

fn create_arena_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let wall_height = 10.0;
    let arena_size = 25.0;
    
    let wall_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, wall_height, arena_size * 2.0)));
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.0, 0.3),
        emissive: Color::srgb(0.1, 0.0, 0.2).into(),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });

    // Four walls
    let wall_positions = vec![
        (Vec3::new(arena_size, wall_height * 0.5, 0.0), Vec3::ZERO),
        (Vec3::new(-arena_size, wall_height * 0.5, 0.0), Vec3::ZERO),
        (Vec3::new(0.0, wall_height * 0.5, arena_size), Vec3::new(0.0, 90.0_f32.to_radians(), 0.0)),
        (Vec3::new(0.0, wall_height * 0.5, -arena_size), Vec3::new(0.0, 90.0_f32.to_radians(), 0.0)),
    ];

    for (position, rotation) in wall_positions {
        commands.spawn(PbrBundle {
            mesh: wall_mesh.clone(),
            material: wall_material.clone(),
            transform: Transform::from_translation(position)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z)),
            ..default()
        });
    }
}

fn update_lighting_effects(
    mut light_query: Query<(&mut PointLight, &mut PsychedelicLight)>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    for (mut point_light, mut psychedelic_light) in light_query.iter_mut() {
        psychedelic_light.time_accumulator += time.delta_seconds();
        
        let time_factor = psychedelic_light.time_accumulator;
        let intensity_factor = config.psychedelic_intensity;
        
        // Color cycling
        let color_phase = time_factor * psychedelic_light.color_shift_speed;
        let r = (color_phase.sin() + 1.0) * 0.5;
        let g = ((color_phase + 2.094).sin() + 1.0) * 0.5; // 120 degrees phase shift
        let b = ((color_phase + 4.188).sin() + 1.0) * 0.5; // 240 degrees phase shift
        
        point_light.color = Color::srgb(r, g, b);
        
        // Intensity pulsing
        let intensity_pulse = (time_factor * psychedelic_light.intensity_pulse_speed).sin();
        point_light.intensity = psychedelic_light.base_intensity 
            * (1.0 + intensity_pulse * 0.5 * intensity_factor);
    }
}

fn update_fog_effects(
    mut graphics_settings: ResMut<GraphicsSettings>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let time_factor = time.elapsed_seconds();
    let intensity = config.psychedelic_intensity;
    
    // Dynamic fog color shifting
    let color_phase = time_factor * 2.0;
    graphics_settings.fog_color = Color::srgb(
        0.1 + (color_phase.sin() * 0.05 * intensity),
        0.0 + ((color_phase + 2.094).sin() * 0.05 * intensity),
        0.2 + ((color_phase + 4.188).sin() * 0.1 * intensity),
    );
    
    // Dynamic fog density
    graphics_settings.fog_density = 0.02 + (time_factor * 0.5).sin() * 0.01 * intensity;
}

// TODO: Implement custom shaders for advanced psychedelic effects
// This would include:
// - Fisheye distortion shader
// - Chromatic aberration
// - Rainbow light trails
// - Kaleidoscope effects
// - Screen-space distortions 