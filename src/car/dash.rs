use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use crate::{track::CarTrack, game_asset::GameAssets};
use crate::input::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct MpsText;

#[derive(Component)]
pub struct KmphText;

#[derive(Component)]
pub struct LapText;

#[derive(Component)]
pub struct TrackPositionText;

#[derive(Component)]
pub struct RideDistanceText;

pub fn dash_fps_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("{:.0}fps", average);
            }
        }
    }
}

pub fn dash_start_system(mut cmd: Commands, game_asset: Res<GameAssets>) {
    let medium: Handle<Font> = game_asset.dash_font.clone();
    let height = Val::Px(90.);
    let width = Val::Px(150.);
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: height.clone(),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let background_color: BackgroundColor = Color::rgba(0.15, 0.15, 0.15, 0.5).into();
        parent
            .spawn(NodeBundle {
                background_color,
                style: Style {
                    width,
                    height: height.clone(),
                    padding: UiRect::all(Val::Px(4.0)),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(4.),
                            left: Val::Px(4.),
                            ..default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: medium.clone(),
                                    font_size: 16.0,
                                    color: Color::YELLOW_GREEN,
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(FpsText);
                parent
                    .spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(20.),
                            left: Val::Px(4.),
                            ..default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: medium.clone(),
                                    font_size: 18.0,
                                    color: Color::SALMON,
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(LapText);
                parent
                    .spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(4.),
                            right: Val::Px(4.),
                            ..default()
                        },
                        text: Text {
                            alignment: TextAlignment::Right,
                            sections: vec![TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: medium.clone(),
                                    font_size: 16.0,
                                    color: Color::YELLOW,
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(TrackPositionText);
                parent
                    .spawn(TextBundle {
                        text: Text {
                            alignment: TextAlignment::Right,
                            sections: vec![TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: medium.clone(),
                                    font_size: 18.0,
                                    color: Color::YELLOW,
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(RideDistanceText);
                parent
                    .spawn(TextBundle {
                        text: Text {
                            alignment: TextAlignment::Right,
                            sections: vec![TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: medium.clone(),
                                    font_size: 24.0,
                                    color: Color::YELLOW_GREEN,
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(MpsText);
                parent
                    .spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: medium.clone(),
                                    font_size: 24.0,
                                    color: Color::YELLOW,
                                },
                            }],
                            ..default()
                        },
                        ..default()
                    })
                    .insert(KmphText);

                #[cfg(feature = "nn")]
                {
                    use bevy_garage_nn::dash::{
                        TrainerEpsilonText, TrainerGenerationText, TrainerRewardsText,
                    };
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                margin: UiRect {
                                    left: Val::Px(4.),
                                    ..default()
                                },
                                top: Val::Px(4.),
                                left: Val::Percent(100.),
                                ..default()
                            },
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 14.0,
                                        color: Color::BLACK,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(TrainerGenerationText);
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                margin: UiRect {
                                    left: Val::Px(4.),
                                    ..default()
                                },
                                top: Val::Px(20.),
                                left: Val::Percent(100.),
                                ..default()
                            },
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 14.0,
                                        color: Color::DARK_GRAY,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(TrainerEpsilonText);
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                margin: UiRect {
                                    left: Val::Px(4.),
                                    ..default()
                                },
                                top: Val::Px(36.),
                                left: Val::Percent(100.),
                                ..default()
                            },
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 14.0,
                                        color: Color::DARK_GRAY,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(TrainerRewardsText);
                }
            });
    });
}

pub fn dash_speed_update_system(
    mut texts: ParamSet<(
        Query<&mut Text, With<MpsText>>,
        Query<&mut Text, With<KmphText>>,
        Query<&mut Text, With<TrackPositionText>>,
        Query<&mut Text, With<RideDistanceText>>,
        Query<&mut Text, With<LapText>>,
    )>,
    mut cars: Query<(&Velocity, &CarTrack, With<PlayerControlled>)>,
) {
    for (velocity, car_track, _) in cars.iter_mut() {
        let mps = velocity.linvel.length();
        let kmph = mps * 3.6;
        texts.p0().single_mut().sections[0].value = format!("{:.1}m/s", mps);
        texts.p1().single_mut().sections[0].value = format!("{:.1}km/h", kmph);

        texts.p2().single_mut().sections[0].value = format!("{:.1}m", car_track.track_position);

        let sign: &str = if car_track.ride_distance.is_sign_negative() {
            "-"
        } else {
            "+"
        };
        texts.p3().single_mut().sections[0].value =
            format!("{sign}{:.1}m", car_track.ride_distance.abs());

        texts.p4().single_mut().sections[0].value = format!("lap {}", car_track.lap);
    }
}
