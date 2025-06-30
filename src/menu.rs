use bevy::prelude::*;
use bevy::app::AppExit;
use crate::{GameState, Player, GameStats, GameConfig};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            setup_main_menu,
        )
        .add_systems(
            Update,
            main_menu_system.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            cleanup_menu,
        )
        .add_systems(
            OnEnter(GameState::Paused),
            setup_pause_menu,
        )
        .add_systems(
            Update,
            (pause_menu_system, handle_unpause_input, update_menu_effects).run_if(in_state(GameState::Paused)),
        )
        .add_systems(
            OnExit(GameState::Paused),
            cleanup_menu,
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            setup_game_over_menu,
        )
        .add_systems(
            Update,
            (game_over_menu_system, update_menu_effects).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            OnExit(GameState::GameOver),
            cleanup_menu,
        )
        .add_systems(
            OnEnter(GameState::Settings),
            setup_settings_menu,
        )
        .add_systems(
            Update,
            (settings_menu_system, update_menu_effects).run_if(in_state(GameState::Settings)),
        )
        .add_systems(
            OnExit(GameState::Settings),
            cleanup_menu,
        )
        .add_systems(
            Update,
            handle_pause_input.run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct MenuButton {
    pub action: ButtonAction,
}

#[derive(Component)]
pub struct MenuTitle;

#[derive(Component)]
pub struct MenuBackground;

#[derive(Component)]
pub struct PsychedelicMenuEffect {
    pub phase: f32,
    pub speed: f32,
}

#[derive(Clone)]
pub enum ButtonAction {
    StartGame,
    ResumeGame,
    RestartGame,
    Settings,
    MainMenu,
    Quit,
    IncreaseVolume,
    DecreaseVolume,
    IncreaseSensitivity,
    DecreaseSensitivity,
    Back,
}

// Main Menu
fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.1, 0.0, 0.2, 0.9).into(),
            ..default()
        },
        MenuUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "VOID",
                TextStyle {
                    font_size: 120.0,
                    color: Color::srgb(1.0, 0.0, 1.0),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }),
        ));

        create_menu_button(parent, "START GAME", ButtonAction::StartGame);
        create_menu_button(parent, "SETTINGS", ButtonAction::Settings);
        create_menu_button(parent, "QUIT", ButtonAction::Quit);
    });
}

// Pause Menu
fn setup_pause_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        },
        MenuUI,
        MenuBackground,
        PsychedelicMenuEffect { phase: 0.0, speed: 0.5 },
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 80.0,
                    color: Color::srgb(1.0, 1.0, 0.0),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }),
            MenuTitle,
            PsychedelicMenuEffect { phase: 0.0, speed: 3.0 },
        ));

        create_menu_button(parent, "RESUME", ButtonAction::ResumeGame);
        create_menu_button(parent, "SETTINGS", ButtonAction::Settings);
        create_menu_button(parent, "MAIN MENU", ButtonAction::MainMenu);
    });
}

// Game Over Menu
fn setup_game_over_menu(
    mut commands: Commands,
    player_query: Query<&Player>,
    game_stats: Res<GameStats>,
) {
    let final_score = if let Ok(player) = player_query.get_single() {
        player.score
    } else {
        0.0
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.2, 0.0, 0.0, 0.9).into(),
            ..default()
        },
        MenuUI,
        MenuBackground,
        PsychedelicMenuEffect { phase: 0.0, speed: 2.0 },
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font_size: 80.0,
                    color: Color::srgb(1.0, 0.0, 0.0),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            }),
            MenuTitle,
            PsychedelicMenuEffect { phase: 0.0, speed: 4.0 },
        ));

        parent.spawn((
            TextBundle::from_section(
                format!("Final Score: {:.0}", final_score),
                TextStyle {
                    font_size: 36.0,
                    color: Color::srgb(1.0, 1.0, 0.0),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            }),
            PsychedelicMenuEffect { phase: 1.57, speed: 2.0 },
        ));

        parent.spawn((
            TextBundle::from_section(
                format!("High Score: {:.0}", game_stats.high_score),
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }),
            PsychedelicMenuEffect { phase: 3.14, speed: 1.5 },
        ));

        create_menu_button(parent, "RESTART", ButtonAction::RestartGame);
        create_menu_button(parent, "MAIN MENU", ButtonAction::MainMenu);
        create_menu_button(parent, "QUIT", ButtonAction::Quit);
    });
}

// Settings Menu
fn setup_settings_menu(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.1, 0.2, 0.9).into(),
            ..default()
        },
        MenuUI,
        MenuBackground,
        PsychedelicMenuEffect { phase: 0.0, speed: 1.0 },
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "SETTINGS",
                TextStyle {
                    font_size: 80.0,
                    color: Color::srgb(0.0, 1.0, 1.0),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }),
            MenuTitle,
            PsychedelicMenuEffect { phase: 0.0, speed: 2.5 },
        ));

        create_setting_row(parent, "Mouse Sensitivity", config.mouse_sensitivity, 
                          ButtonAction::DecreaseSensitivity, ButtonAction::IncreaseSensitivity);

        create_setting_row(parent, "Movement Speed", config.movement_speed,
                          ButtonAction::DecreaseVolume, ButtonAction::IncreaseVolume);

        parent.spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            create_menu_button(parent, "BACK", ButtonAction::Back);
        });
    });
}

fn create_menu_button(parent: &mut ChildBuilder, text: &str, action: ButtonAction) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(60.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: Color::srgba(0.2, 0.0, 0.4, 0.8).into(),
            border_color: Color::srgb(1.0, 0.0, 1.0).into(),
            ..default()
        },
        MenuButton { action },
        MenuUI,
        PsychedelicMenuEffect { phase: 0.0, speed: 1.0 },
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            MenuUI,
        ));
    });
}

fn create_setting_row(parent: &mut ChildBuilder, label: &str, value: f32, 
                     decrease_action: ButtonAction, increase_action: ButtonAction) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Px(80.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.3, 0.6).into(),
            border_color: Color::srgb(0.5, 0.5, 1.0).into(),
            ..default()
        },
        MenuUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            MenuUI,
        ));

        parent.spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            create_small_button(parent, "-", decrease_action.clone());

            parent.spawn((
                TextBundle::from_section(
                    format!("{:.2}", value),
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                        ..default()
                    },
                ),
                MenuUI,
            ));

            create_small_button(parent, "+", increase_action);
        });
    });
}

fn create_small_button(parent: &mut ChildBuilder, text: &str, action: ButtonAction) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                margin: UiRect::horizontal(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.3, 0.0, 0.6, 0.8).into(),
            border_color: Color::srgb(1.0, 0.0, 1.0).into(),
            ..default()
        },
        MenuButton { action },
        MenuUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            MenuUI,
        ));
    });
}

// Menu Systems
fn main_menu_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button.action {
                ButtonAction::StartGame => next_state.set(GameState::InGame),
                ButtonAction::Settings => next_state.set(GameState::Settings),
                ButtonAction::Quit => { exit.send(AppExit::Success); },
                _ => {}
            }
        }
    }
}

fn pause_menu_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button.action {
                ButtonAction::ResumeGame => next_state.set(GameState::InGame),
                ButtonAction::Settings => next_state.set(GameState::Settings),
                ButtonAction::MainMenu => next_state.set(GameState::MainMenu),
                _ => {}
            }
        }
    }
}

fn game_over_menu_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button.action {
                ButtonAction::RestartGame => next_state.set(GameState::InGame),
                ButtonAction::MainMenu => next_state.set(GameState::MainMenu),
                ButtonAction::Quit => { exit.send(AppExit::Success); },
                _ => {}
            }
        }
    }
}

fn settings_menu_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut config: ResMut<GameConfig>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button.action {
                ButtonAction::IncreaseSensitivity => {
                    config.mouse_sensitivity = (config.mouse_sensitivity + 0.001).min(0.01);
                }
                ButtonAction::DecreaseSensitivity => {
                    config.mouse_sensitivity = (config.mouse_sensitivity - 0.001).max(0.0001);
                }
                ButtonAction::IncreaseVolume => {
                    config.movement_speed = (config.movement_speed + 0.5).min(15.0);
                }
                ButtonAction::DecreaseVolume => {
                    config.movement_speed = (config.movement_speed - 0.5).max(1.0);
                }
                ButtonAction::Back => next_state.set(GameState::MainMenu),
                _ => {}
            }
        }
    }
}

fn handle_pause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn handle_unpause_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}

fn update_menu_effects(
    mut menu_query: Query<(&mut PsychedelicMenuEffect, &mut BackgroundColor), With<MenuBackground>>,
    mut title_query: Query<(&mut PsychedelicMenuEffect, &mut Text), (With<MenuTitle>, Without<MenuBackground>)>,
    mut effect_query: Query<(&mut PsychedelicMenuEffect, &mut Text), (Without<MenuTitle>, Without<MenuBackground>)>,
    time: Res<Time>,
) {
    // Update background effects
    for (mut effect, mut background_color) in menu_query.iter_mut() {
        effect.phase += time.delta_seconds() * effect.speed;
        let pulse = (effect.phase.sin() + 1.0) * 0.5;
        let color_shift = (effect.phase * 0.5).cos();
        
        background_color.0 = Color::srgba(
            0.1 + pulse * 0.1,
            color_shift.abs() * 0.1,
            0.2 + pulse * 0.1,
            0.9,
        );
    }

    // Update title effects
    for (mut effect, mut text) in title_query.iter_mut() {
        effect.phase += time.delta_seconds() * effect.speed;
        let color_phase = effect.phase;
        
        if let Some(section) = text.sections.get_mut(0) {
            section.style.color = Color::srgb(
                (color_phase.sin() + 1.0) * 0.5,
                ((color_phase + 2.094).sin() + 1.0) * 0.5,
                ((color_phase + 4.188).sin() + 1.0) * 0.5,
            );
        }
    }

    // Update other text effects
    for (mut effect, mut text) in effect_query.iter_mut() {
        effect.phase += time.delta_seconds() * effect.speed;
        let pulse = (effect.phase.sin() + 1.0) * 0.5;
        
        if let Some(section) = text.sections.get_mut(0) {
            let base_srgba = section.style.color.to_srgba();
            section.style.color = Color::srgb(
                base_srgba.red * (0.5 + pulse * 0.5),
                base_srgba.green * (0.5 + pulse * 0.5),
                base_srgba.blue * (0.5 + pulse * 0.5),
            );
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuUI>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
} 