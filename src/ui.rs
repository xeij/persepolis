use bevy::prelude::*;
use crate::{Player, GameConfig};

#[cfg(feature = "dev")]
use iyes_perf_ui::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, (
                update_health_bar,
                update_score_display,
                update_kill_count_display,
                update_crosshair,
                update_ui_effects,
            ));
            
        #[cfg(feature = "dev")]
        app.add_systems(Startup, setup_perf_ui);
    }
}

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct KillCountText;

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct PsychedelicUI;

fn setup_ui(mut commands: Commands) {
    // Root UI node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Health bar
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(20.0),
                        bottom: Val::Px(20.0),
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.0, 0.2).into(),
                    border_color: Color::srgb(1.0, 0.0, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::srgb(1.0, 0.0, 0.5).into(),
                            ..default()
                        },
                        HealthBar,
                    ));
                });

            // Score display
            parent.spawn((
                TextBundle::from_section(
                    "Score: 0",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::srgb(1.0, 1.0, 0.0),
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                }),
                ScoreText,
                PsychedelicUI,
            ));

            // Kill count display
            parent.spawn((
                TextBundle::from_section(
                    "Kills: 0",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.0, 1.0, 0.5),
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(60.0),
                    left: Val::Px(20.0),
                    ..default()
                }),
                KillCountText,
                PsychedelicUI,
            ));

            // Crosshair
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(50.0),
                        top: Val::Percent(50.0),
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        margin: UiRect {
                            left: Val::Px(-10.0),
                            top: Val::Px(-10.0),
                            ..default()
                        },
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                    border_color: Color::srgb(1.0, 1.0, 1.0).into(),
                    ..default()
                },
                Crosshair,
                PsychedelicUI,
            ));
        });
}

#[cfg(feature = "dev")]
fn setup_perf_ui(mut commands: Commands) {
    commands.spawn((
        PerfUiCompleteBundle::default(),
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            right: Val::Px(20.0),
            ..default()
        },
    ));
}

fn update_health_bar(
    player_query: Query<&Player>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
) {
    if let (Ok(player), Ok(mut style)) = 
        (player_query.get_single(), health_bar_query.get_single_mut()) {
        let health_percent = (player.health / player.max_health) * 100.0;
        style.width = Val::Percent(health_percent);
    }
}

fn update_score_display(
    player_query: Query<&Player>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
) {
    if let (Ok(player), Ok(mut text)) = 
        (player_query.get_single(), score_text_query.get_single_mut()) {
        text.sections[0].value = format!("Score: {:.0}", player.score);
    }
}

fn update_kill_count_display(
    player_query: Query<&Player>,
    mut kill_count_query: Query<&mut Text, With<KillCountText>>,
) {
    if let (Ok(player), Ok(mut text)) = 
        (player_query.get_single(), kill_count_query.get_single_mut()) {
        text.sections[0].value = format!("Kills: {}", player.kill_count);
    }
}

fn update_crosshair(
    player_query: Query<&Player>,
    mut crosshair_query: Query<&mut BorderColor, With<Crosshair>>,
    time: Res<Time>,
) {
    if let (Ok(player), Ok(mut border_color)) = 
        (player_query.get_single(), crosshair_query.get_single_mut()) {
        
        if player.is_shooting {
            // Pulsing red when shooting
            let pulse = (time.elapsed_seconds() * 10.0).sin();
            border_color.0 = Color::srgb(1.0, pulse.abs(), pulse.abs());
        } else {
            // Normal white crosshair
            border_color.0 = Color::srgb(1.0, 1.0, 1.0);
        }
    }
}

fn update_ui_effects(
    mut text_query: Query<&mut Text, With<PsychedelicUI>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let time_factor = time.elapsed_seconds();
    let intensity = config.psychedelic_intensity;
    
    for mut text in text_query.iter_mut() {
        let color_phase = time_factor * 3.0;
        let r = (color_phase.sin() + 1.0) * 0.5;
        let g = ((color_phase + 2.094).sin() + 1.0) * 0.5;
        let b = ((color_phase + 4.188).sin() + 1.0) * 0.5;
        
        // Mix with base color based on intensity
        let base_color = text.sections[0].style.color;
        let psychedelic_color = Color::srgb(r, g, b);
        
        text.sections[0].style.color = Color::srgb(
            base_color.to_srgba().red * (1.0 - intensity * 0.5) + psychedelic_color.to_srgba().red * intensity * 0.5,
            base_color.to_srgba().green * (1.0 - intensity * 0.5) + psychedelic_color.to_srgba().green * intensity * 0.5,
            base_color.to_srgba().blue * (1.0 - intensity * 0.5) + psychedelic_color.to_srgba().blue * intensity * 0.5,
        );
    }
} 