use bevy::prelude::*;
use crate::GameConfig;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<GameConfig>()
            .init_resource::<GameStats>()
            .init_resource::<SteamConfig>()
            .add_systems(Startup, setup_game_state)
            .add_systems(Update, (
                update_game_stats,
                handle_game_over_condition,
                update_high_score,
            ));
    }
}

// Proper Bevy states
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    GameOver,
    Settings,
}

#[derive(Resource)]
pub struct GameStats {
    pub total_kills: u32,
    pub total_score: f32,
    pub high_score: f32,
    pub games_played: u32,
    pub time_played: f32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            total_kills: 0,
            total_score: 0.0,
            high_score: 0.0,
            games_played: 0,
            time_played: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct SteamConfig {
    pub app_id: u32,
    pub achievements_enabled: bool,
    pub leaderboards_enabled: bool,
}

impl Default for SteamConfig {
    fn default() -> Self {
        Self {
            app_id: 0, // TODO: Replace with actual Steam App ID
            achievements_enabled: false,
            leaderboards_enabled: false,
        }
    }
}

fn setup_game_state(_commands: Commands) {
    println!("Game State System Initialized");
    println!("Controls:");
    println!("- WASD: Move");
    println!("- Mouse: Look around");
    println!("- Left Click: Grab cursor and shoot");
    println!("- Escape: Pause/Menu");
    println!("- Shift: Sprint");
}

fn update_game_stats(
    mut game_stats: ResMut<GameStats>,
    time: Res<Time>,
    state: Res<State<GameState>>,
) {
    if *state.get() == GameState::InGame {
        game_stats.time_played += time.delta_seconds();
    }
}

fn handle_game_over_condition(
    player_query: Query<&crate::Player>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if *state.get() == GameState::InGame {
        if let Ok(player) = player_query.get_single() {
            if player.health <= 0.0 {
                next_state.set(GameState::GameOver);
            }
        }
    }
}

fn update_high_score(
    player_query: Query<&crate::Player>,
    mut game_stats: ResMut<GameStats>,
    state: Res<State<GameState>>,
) {
    if *state.get() == GameState::GameOver {
        if let Ok(player) = player_query.get_single() {
            if player.score > game_stats.high_score {
                game_stats.high_score = player.score;
            }
        }
    }
}

// TODO: Steam integration functions
#[allow(dead_code)]
fn initialize_steam() -> Result<(), String> {
    // This would initialize the Steam SDK
    // steamworks::Client::init()
    println!("Steam integration not yet implemented");
    Ok(())
}

#[allow(dead_code)]
fn unlock_achievement(_achievement_name: &str) {
    // This would unlock Steam achievements
    println!("Achievement unlocked: {}", _achievement_name);
}

#[allow(dead_code)]
fn submit_score_to_leaderboard(_score: f32) {
    // This would submit scores to Steam leaderboards
    println!("Score submitted to leaderboard: {}", _score);
} 