use bevy::prelude::*;
// use bevy_kira_audio::prelude::*;

use crate::ui::window::*;
use crate::*;

mod game_over;
mod settings;
mod window;

static PRESS_START_TEXT: &str = "Press space to start";

fn ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // game_controller: Res<GameController>,
) {
    // text ui
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // FPS text
            parent
                .spawn_bundle(
                    TextBundle::from_sections([
                        TextSection::new(
                            "FPS: ",
                            TextStyle {
                                font: asset_server.load(FONT_PATH),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 30.0,
                            color: Color::GOLD,
                        }),
                    ])
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Percent(0.0),
                            left: Val::Percent(0.0),
                            ..Default::default()
                        },
                        ..default()
                    }),
                )
                .insert(UiText {
                    text_type: UiTextType::FPSText,
                })
                .insert(UiZ(20.0));

            // Start text
            parent
                .spawn_bundle(TextBundle::from_section(
                    PRESS_START_TEXT,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                ))
                .insert(UiText {
                    text_type: UiTextType::StartMessage,
                })
                .insert(UiZ(20.0));

            // Score text
            parent
                .spawn_bundle(
                    TextBundle::from_sections([TextSection::from_style(TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 80.0,
                        color: Color::BLACK,
                    })])
                    .with_style(Style {
                        margin: UiRect {
                            top: Val::Percent(10.0),
                            bottom: Val::Percent(15.0),
                            ..Default::default()
                        },
                        ..default()
                    }),
                )
                .insert(UiText {
                    text_type: UiTextType::Score,
                })
                .insert(UiZ(20.0));
        });
}

fn text_ui_system(
    mut query: Query<(&mut Text, &mut Visibility, &UiText)>,
    game_controller: Res<GameController>,
    diagnostics: Res<Diagnostics>,
) {
    const HIGH_SCORE_TEXT: &str = "High Score";

    for (mut text, mut visibility, ui_text) in query.iter_mut() {
        match ui_text.text_type {
            UiTextType::StartMessage => {
                if game_controller.has_game_started() {
                    visibility.is_visible = false;
                } else {
                    visibility.is_visible = true;
                }
            }
            UiTextType::Score => {
                if game_controller.has_game_started() {
                    text.sections[0].value = game_controller.score.to_string();
                } else {
                    text.sections[0].value = format!(
                        "{}: {}",
                        HIGH_SCORE_TEXT, game_controller.player_stats.high_score
                    );
                }
            }
            UiTextType::FPSText => {
                if game_controller.settings.show_fps {
                    visibility.is_visible = true;

                    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(average) = fps.average() {
                            // Update the value of the second section
                            text.sections[1].value = format!("{average:.2}");
                        }
                    }
                } else {
                    visibility.is_visible = false;
                }
            }
        }
    }
}

#[derive(Component)]
struct UiText {
    text_type: UiTextType,
}

enum UiTextType {
    StartMessage,
    Score,
    FPSText,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiZPlugin)
            .add_startup_system(ui_setup)
            .add_system(text_ui_system)
            .add_plugin(settings::SettingsPlugin)
            .add_plugin(game_over::GameOverUiPlugin);
    }
}

pub struct UiZPlugin; // TODO: fix bevy ui
#[derive(Component)]
pub struct UiZ(pub f32);

impl Plugin for UiZPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            update_uiz.after(bevy::transform::TransformSystem::TransformPropagate),
        );
    }
}

#[allow(clippy::type_complexity)]
fn update_uiz(mut query: Query<(&UiZ, &mut GlobalTransform), (With<Node>, Changed<Transform>)>) {
    for (uiz, mut transform) in query.iter_mut() {
        let translation = transform.translation_mut();
        translation.z = uiz.0;
    }
}
