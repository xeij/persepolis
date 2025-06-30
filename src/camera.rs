use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::{GameCamera, GameConfig};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    mouse_look,
                    handle_cursor_grab,
                    update_camera_effects,
                ),
            );
    }
}

#[derive(Component)]
pub struct FirstPersonCamera {
    pub pitch: f32,
    pub yaw: f32,
    pub distortion_strength: f32,
    pub chromatic_aberration: f32,
    pub time_accumulator: f32,
}

impl Default for FirstPersonCamera {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            distortion_strength: 0.0,
            chromatic_aberration: 0.0,
            time_accumulator: 0.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn the first-person camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // Wide FOV for psychedelic effect
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 110.0_f32.to_radians(), 
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.8, 0.0), // Eye height
            ..default()
        },
        GameCamera,
        FirstPersonCamera::default(),
    ));
}

fn mouse_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut FirstPersonCamera), With<GameCamera>>,
    config: Res<GameConfig>,
) {
    if let Ok((mut transform, mut camera)) = camera_query.get_single_mut() {
        let mut delta = Vec2::ZERO;
        for motion in mouse_motion.read() {
            delta += motion.delta;
        }

        // Apply mouse sensitivity
        delta *= config.mouse_sensitivity;

        // Update yaw and pitch
        camera.yaw -= delta.x;
        camera.pitch -= delta.y;

        // Clamp pitch to prevent camera flipping
        camera.pitch = camera.pitch.clamp(-1.5, 1.5);

        // Apply rotation
        transform.rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw)
            * Quat::from_axis_angle(Vec3::X, camera.pitch);
    }
}

fn handle_cursor_grab(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        if mouse.just_pressed(MouseButton::Left) {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }

        if key.just_pressed(KeyCode::Escape) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn update_camera_effects(
    time: Res<Time>,
    mut camera_query: Query<&mut FirstPersonCamera>,
    config: Res<GameConfig>,
) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        camera.time_accumulator += time.delta_seconds();
        
        // Psychedelic distortion based on intensity
        let intensity = config.psychedelic_intensity;
        camera.distortion_strength = intensity * (camera.time_accumulator * 2.0).sin() * 0.1;
        camera.chromatic_aberration = intensity * (camera.time_accumulator * 3.0).cos() * 0.02;
    }
} 