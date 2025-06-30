use bevy::prelude::*;
use bevy::window::WindowResolution;

#[cfg(feature = "dev")]
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use iyes_perf_ui::prelude::*;

mod camera;
mod player;
mod zombies;
mod weapons;
mod graphics;
mod audio;
mod ui;
mod physics;
mod game_state;
mod particles;
mod menu;

use camera::*;
use player::*;
use zombies::*;
use weapons::*;
use graphics::*;
use audio::*;
use ui::*;
use physics::*;
use game_state::*;
use particles::*;
use menu::*;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "void".into(),
                resolution: WindowResolution::new(1920., 1080.),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            // Core game plugins
            CameraPlugin,
            PlayerPlugin,
            ZombiePlugin,
            WeaponsPlugin,
            GraphicsPlugin,
            AudioPlugin,
            UIPlugin,
            PhysicsPlugin,
            GameStatePlugin,
            ParticlePlugin,
            MenuPlugin,
            // Performance UI
            PerfUiPlugin,
        ));

    #[cfg(feature = "dev")]
    app.add_plugins(DefaultInspectorConfigPlugin);
    
    app.run();
}

#[derive(Component)]
pub struct GameCamera;

#[derive(Resource)]
pub struct GameConfig {
    pub mouse_sensitivity: f32,
    pub movement_speed: f32,
    pub psychedelic_intensity: f32,
    pub zombie_spawn_rate: f32,
    pub score_multiplier: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.002,
            movement_speed: 5.0,
            psychedelic_intensity: 1.0,
            zombie_spawn_rate: 1.0,
            score_multiplier: 1.0,
        }
    }
} 