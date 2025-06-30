use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_audio)
            .add_systems(Update, (
                handle_shooting_sounds,
                handle_ambient_audio,
            ));
    }
}

#[derive(Resource)]
pub struct GameAudio {
    pub shoot_sound: Handle<AudioSource>,
    pub zombie_death_sound: Handle<AudioSource>,
    pub ambient_track: Handle<AudioSource>,
    pub player_damage_sound: Handle<AudioSource>,
}

fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // TODO: Load actual audio files
    // For now, we'll use placeholder handles
    let game_audio = GameAudio {
        shoot_sound: asset_server.load("sounds/shoot.ogg"),
        zombie_death_sound: asset_server.load("sounds/zombie_death.ogg"),
        ambient_track: asset_server.load("sounds/ambient.ogg"),
        player_damage_sound: asset_server.load("sounds/player_damage.ogg"),
    };
    
    commands.insert_resource(game_audio);
    
    println!("Audio system initialized - add sound files to assets/sounds/");
}

fn handle_shooting_sounds(
    // TODO: Implement audio systems once we have sound files
) {
    // This would trigger shoot sounds when player shoots
}

fn handle_ambient_audio(
    // TODO: Implement ambient audio system
) {
    // This would manage background music and ambient sounds
} 