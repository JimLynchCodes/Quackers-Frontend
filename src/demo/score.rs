use bevy::prelude::*;
use serde::Deserialize;

use super::websocket_connect::{UpdateLeaderboardBevyEvent, UpdateYourScoreBevyEvent};

#[derive(Component)]
struct ErrorTextComponent;

#[derive(Component)]
struct YourScoreText;

#[derive(Component)]
struct YourPositionText;

#[derive(Component)]
struct YourLeaderboardPlaceText;

#[derive(Component)]
struct LeaderboardName1stPlaceText;
#[derive(Component)]
struct LeaderboardName2ndPlaceText;
#[derive(Component)]
struct LeaderboardName3rdPlaceText;
#[derive(Component)]
struct LeaderboardName4thPlaceText;
#[derive(Component)]
struct LeaderboardName5thPlaceText;

#[derive(Component)]
struct LeaderboardScore1stPlaceText;
#[derive(Component)]
struct LeaderboardScore2ndPlaceText;
#[derive(Component)]
struct LeaderboardScore3rdPlaceText;
#[derive(Component)]
struct LeaderboardScore4thPlaceText;
#[derive(Component)]
struct LeaderboardScore5thPlaceText;

#[derive(Debug, Deserialize)]
pub struct LeaderboardUpdateData {
    pub your_points: u64,
    pub your_leaderboard_place: u64,

    pub leaderboard_name_1st_place: String,
    pub leaderboard_name_2nd_place: String,
    pub leaderboard_name_3rd_place: String,
    pub leaderboard_name_4th_place: String,
    pub leaderboard_name_5th_place: String,

    pub leaderboard_score_1st_place: u64,
    pub leaderboard_score_2nd_place: u64,
    pub leaderboard_score_3rd_place: u64,
    pub leaderboard_score_4th_place: u64,
    pub leaderboard_score_5th_place: u64,
}

const TABLE_OFFSET: u32 = 30;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_score_text);
    app.add_systems(Startup, setup_leaderboard_table);
    app.add_systems(Update, bevy_event_listener_update_your_score_text);
    app.add_systems(Update, bevy_event_listener_update_leaderboard);
}

fn create_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("creating Score text...");

    let score_text = TextBundle {
        style: Style {
            position_type: PositionType::Absolute, // Absolute positioning
            left: Val::Percent(3.),
            top: Val::Percent(3.),
            ..Default::default()
        },
        text: Text::from_section(
            "Score: 0".to_string(),
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"), // Load your font here
                font_size: 25.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    };

    let position_text = TextBundle {
        style: Style {
            position_type: PositionType::Absolute, // Absolute positioning
            left: Val::Percent(3.),
            top: Val::Percent(6.),
            ..Default::default()
        },
        text: Text::from_section(
            "Position: --".to_string(),
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"), // Load your font here
                font_size: 25.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    };

    commands.spawn(score_text).insert(YourScoreText);
    commands.spawn(position_text).insert(YourPositionText);
}

fn bevy_event_listener_update_your_score_text(
    mut event_reader: EventReader<UpdateYourScoreBevyEvent>,
    mut score_text: Query<&mut Text, With<YourScoreText>>,
) {
    for e in event_reader.read() {
        info!("heard the update score event!");

        for mut text in score_text.iter_mut() {
            text.sections[0].value = format!("Score: {}", e.new_score);
        }
    }
}

// mut param_set: ParamSet<(
//     Query<&mut Transform, With<_CrackerComponent>>,
//     Query<&mut Transform, With<_CrackerText>>,
//     Query<&mut Text, With<_CrackerText>>,
// )>,

// mut param_set: ParamSet<(
//     Query<&mut Text, With<LeaderboardName1stPlaceText>>,
//     Query<&mut Text, With<LeaderboardName2ndPlaceText>>,
//     Query<&mut Text, With<LeaderboardName3rdPlaceText>>,
//     Query<&mut Text, With<LeaderboardName4thPlaceText>>,
//     Query<&mut Text, With<LeaderboardName5thPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore1stPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore2ndPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore3rdPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore4thPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore5thPlaceText>>,
// )>,

// mut leaderboard_name_1st_place_text: Query<&mut Text, With<LeaderboardName1stPlaceText>>,
// mut leaderboard_name_2nd_place_text: Query<&mut Text, With<LeaderboardName2ndPlaceText>>,
// mut leaderboard_name_3rd_place_text: Query<&mut Text, With<LeaderboardName3rdPlaceText>>,
// mut leaderboard_name_4th_place_text: Query<&mut Text, With<LeaderboardName4thPlaceText>>,
// mut leaderboard_name_5th_place_text: Query<&mut Text, With<LeaderboardName5thPlaceText>>,
// mut leaderboard_score_1st_place_text: Query<&mut Text, With<LeaderboardScore1stPlaceText>>,
// mut leaderboard_score_2nd_place_text: Query<&mut Text, With<LeaderboardScore2ndPlaceText>>,
// mut leaderboard_score_3rd_place_text: Query<&mut Text, With<LeaderboardScore3rdPlaceText>>,
// mut leaderboard_score_4th_place_text: Query<&mut Text, With<LeaderboardScore4thPlaceText>>,
// mut leaderboard_score_5th_place_text: Query<&mut Text, With<LeaderboardScore5thPlaceText>>,

// mut your_position_text: Query<&mut Text, With<YourPositionText>>,
// mut event_reader: EventReader<UpdateLeaderboardBevyEvent>,
// mut param_set: ParamSet<(
//     Query<&mut Text, With<YourPositionText>>,
//     Query<&mut Text, With<LeaderboardName1stPlaceText>>,
//     Query<&mut Text, With<LeaderboardName2ndPlaceText>>,
//     Query<&mut Text, With<LeaderboardName3rdPlaceText>>,
//     Query<&mut Text, With<LeaderboardName4thPlaceText>>,
//     Query<&mut Text, With<LeaderboardName5thPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore1stPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore2ndPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore3rdPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore4thPlaceText>>,
//     Query<&mut Text, With<LeaderboardScore5thPlaceText>>,
// )>,

// fn bevy_event_listener_update_leaderboard(
//     mut leaderboard_name_1st_place_text: Query<&mut Text, With<LeaderboardName1stPlaceText>>,
//     mut leaderboard_name_2nd_place_text: Query<&mut Text, With<LeaderboardName2ndPlaceText>>,
//     mut leaderboard_name_3rd_place_text: Query<&mut Text, With<LeaderboardName3rdPlaceText>>,
//     mut leaderboard_name_4th_place_text: Query<&mut Text, With<LeaderboardName4thPlaceText>>,
//     mut leaderboard_name_5th_place_text: Query<&mut Text, With<LeaderboardName5thPlaceText>>,
//     mut leaderboard_score_1st_place_text: Query<&mut Text, With<LeaderboardScore1stPlaceText>>,
//     mut leaderboard_score_2nd_place_text: Query<&mut Text, With<LeaderboardScore2ndPlaceText>>,
//     mut leaderboard_score_3rd_place_text: Query<&mut Text, With<LeaderboardScore3rdPlaceText>>,
//     mut leaderboard_score_4th_place_text: Query<&mut Text, With<LeaderboardScore4thPlaceText>>,
//     mut leaderboard_score_5th_place_text: Query<&mut Text, With<LeaderboardScore5thPlaceText>>,
// ) {

fn bevy_event_listener_update_leaderboard(
    mut event_reader: EventReader<UpdateLeaderboardBevyEvent>,
    mut text: Query<&mut Text>,
    your_position_entity: Query<Entity, With<YourPositionText>>,
    place_1_name: Query<Entity, With<LeaderboardName1stPlaceText>>,
    place_2_name: Query<Entity, With<LeaderboardName2ndPlaceText>>,
    place_3_name: Query<Entity, With<LeaderboardName3rdPlaceText>>,
    place_4_name: Query<Entity, With<LeaderboardName4thPlaceText>>,
    place_5_name: Query<Entity, With<LeaderboardName5thPlaceText>>,
    place_1_score: Query<Entity, With<LeaderboardScore1stPlaceText>>,
    place_2_score: Query<Entity, With<LeaderboardScore2ndPlaceText>>,
    place_3_score: Query<Entity, With<LeaderboardScore3rdPlaceText>>,
    place_4_score: Query<Entity, With<LeaderboardScore4thPlaceText>>,
    place_5_score: Query<Entity, With<LeaderboardScore5thPlaceText>>,
) {
    for e in event_reader.read() {
        info!("heard the update score event!");

        let default_leaderboard_data = LeaderboardUpdateData {
            your_points: 0,
            your_leaderboard_place: 0,

            leaderboard_name_1st_place: "-- ".to_string(),
            leaderboard_name_2nd_place: "-- ".to_string(),
            leaderboard_name_3rd_place: "-- ".to_string(),
            leaderboard_name_4th_place: "-- ".to_string(),
            leaderboard_name_5th_place: "-- ".to_string(),

            leaderboard_score_1st_place: 0,
            leaderboard_score_2nd_place: 0,
            leaderboard_score_3rd_place: 0,
            leaderboard_score_4th_place: 0,
            leaderboard_score_5th_place: 0,
        };

        let update_leaderboard_msg_data =
            serde_json::from_value(e.data.clone()).unwrap_or_else(|op| {
                info!("Failed to parse incoming websocket message: {}", op);
                default_leaderboard_data
            });

        let mut your_position_text = text.get_mut(your_position_entity.single()).unwrap();
        your_position_text.sections[0].value =
            format_leaderboard_place(update_leaderboard_msg_data.your_leaderboard_place);

        let first_place_name = update_leaderboard_msg_data.leaderboard_name_1st_place;
        let second_place_name = update_leaderboard_msg_data.leaderboard_name_2nd_place;
        let third_place_name = update_leaderboard_msg_data.leaderboard_name_3rd_place;
        let fourth_place_name = update_leaderboard_msg_data.leaderboard_name_4th_place;
        let fifth_place_name = update_leaderboard_msg_data.leaderboard_name_5th_place;

        let no_name = "--".to_string();

        let no_first_place_name = first_place_name == "--";
        let no_second_place_name = second_place_name == "--";
        let no_third_place_name = third_place_name == "--";
        let no_fourth_place_name = fourth_place_name == "--";
        let no_fifth_place_name = fifth_place_name == "--";

        // name texts
        let mut place_1_name_text = text.get_mut(place_1_name.single()).unwrap();
        place_1_name_text.sections[0].value = if no_first_place_name { no_name.clone() } else {
            format!("{} - ", first_place_name)
        };

        let mut place_2_name_text = text.get_mut(place_2_name.single()).unwrap();
        place_2_name_text.sections[0].value =if no_second_place_name { no_name.clone() } else {
            format!("{} - ", second_place_name)
        };
        let mut place_3_name_text = text.get_mut(place_3_name.single()).unwrap();
        place_3_name_text.sections[0].value = if no_third_place_name { no_name.clone() } else {
            format!("{} - ", third_place_name)
        };

        let mut place_4_name_text = text.get_mut(place_4_name.single()).unwrap();
        place_4_name_text.sections[0].value =if no_fourth_place_name { no_name.clone() } else {
            format!("{} - ", fourth_place_name)
        };

        let mut place_5_name_text = text.get_mut(place_5_name.single()).unwrap();
        place_5_name_text.sections[0].value = if no_fifth_place_name { no_name.clone() } else {
            format!("{} - ", fifth_place_name)
        };
            //score texts
        let mut place_1_score_text = text.get_mut(place_1_score.single()).unwrap();  
        place_1_score_text.sections[0].value = if no_first_place_name {
            "".to_string()
        }
        else {
            update_leaderboard_msg_data
            .leaderboard_score_1st_place
            .to_string()
        };

        let mut place_2_score_text = text.get_mut(place_2_score.single()).unwrap();
        place_2_score_text.sections[0].value = if no_second_place_name {
            "".to_string()
        }
        else {
            update_leaderboard_msg_data
            .leaderboard_score_2nd_place
            .to_string()
        };

        let mut place_3_score_text = text.get_mut(place_3_score.single()).unwrap();
        place_3_score_text.sections[0].value = if no_third_place_name {
            "".to_string()
        }
        else {
            update_leaderboard_msg_data
            .leaderboard_score_3rd_place
            .to_string()
        };

        let mut place_4_score_text = text.get_mut(place_4_score.single()).unwrap();
        place_4_score_text.sections[0].value = if no_fourth_place_name {
            "".to_string()
        }
        else {
            update_leaderboard_msg_data
            .leaderboard_score_4th_place
            .to_string()
        };

        let mut place_5_score_text = text.get_mut(place_5_score.single()).unwrap();
        place_5_score_text.sections[0].value = if no_fifth_place_name {
            "".to_string()
        }
        else {
            update_leaderboard_msg_data
            .leaderboard_score_5th_place
            .to_string()
        };

        // text.sections[0].value = format!("Position: {:?}", update_leaderboard_msg_data.your_leaderboard_place.clone());
        //     // Update your position

        // param_set.p0()

        // for mut text in param_set.q0().iter_mut() {
        //     text.sections[0].value = format!("Position: {:?}", update_leaderboard_msg_data.your_leaderboard_place.clone());
        // }

        // // update the 5 name fields
        // for mut text in leaderboard_name_1st_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_1st_place.clone();
        // }

        // for mut text in leaderboard_name_2nd_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_2nd_place.clone();
        // }

        // for mut text in leaderboard_name_3rd_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_3rd_place.clone();
        // }

        // for mut text in leaderboard_name_4th_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_4th_place.clone();
        // }

        // for mut text in leaderboard_name_5th_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_5th_place.clone();

        // // update the 5 score fields
        // for mut text in leaderboard_score_1st_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_1st_place.to_string();
        // }

        // for mut text in leaderboard_score_2nd_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_2nd_place.to_string();
        // }

        // for mut text in leaderboard_score_3rd_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_3rd_place.to_string();
        // }

        // for mut text in leaderboard_score_4th_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_4th_place.to_string();
        // }

        // for mut text in leaderboard_score_5th_place_text.iter_mut() {
        //     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_5th_place.to_string();
        // }
        // }
    }
}

fn format_leaderboard_place(leaderboard_position: u64) -> String {
    let suffix = match leaderboard_position % 100 {
        11 | 12 | 13 => "th", // Special case for 11th, 12th, 13th
        _ => match leaderboard_position % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    format!("{}{} Place", leaderboard_position, suffix)
}

// fn bevy_event_listener_update_leaderboard3(
//     mut event_reader: EventReader<UpdateLeaderboardBevyEvent>,
//     mut text: Query<&mut Text>,
//     place_1_name: Query<Entity, With<LeaderboardName1stPlaceText>>,
//     place_2_name: Query<Entity, With<LeaderboardName2ndPlaceText>>,
//     place_3_name: Query<Entity, With<LeaderboardName3rdPlaceText>>,
//     place_4_name: Query<Entity, With<LeaderboardName4thPlaceText>>,
//     place_5_name: Query<Entity, With<LeaderboardName5thPlaceText>>,
//     // You can do the same as above with the other texts that you want to access
//     // Idk what this is, but it can prolly be done the same with
//     // mut your_position_text: Query<&mut Text, With<YourPositionText>>,
// ) {
//     // Then to get the first place name:
//     let extracted_place_1_name = text.get_mut(place_1_name.single());
//     let extracted_place_2_name = text.get_mut(place_2_name.single());
//     // Rest of your code here
// }

// fn bevy_event_listener_update_leaderboard2(// mut your_position_text: Query<&mut Text, With<YourPositionText>>,
// mut event_reader: EventReader<UpdateLeaderboardBevyEvent>,
// mut leaderboard_name_1st_place_text: Query<&mut Text, (With<LeaderboardName1stPlaceText>, Without<LeaderboardScore1stPlaceText>)>,
// mut leaderboard_name_2nd_place_text: Query<&mut Text, (With<LeaderboardName2ndPlaceText>, Without<LeaderboardScore2ndPlaceText>)>,
// mut leaderboard_name_3rd_place_text: Query<&mut Text, (With<LeaderboardName3rdPlaceText>, Without<LeaderboardScore3rdPlaceText>)>,
// mut leaderboard_name_4th_place_text: Query<&mut Text, (With<LeaderboardName4thPlaceText>, Without<LeaderboardScore4thPlaceText>)>,
// mut leaderboard_name_5th_place_text: Query<&mut Text, (With<LeaderboardName5thPlaceText>, Without<LeaderboardScore5thPlaceText>)>,
// mut leaderboard_score_1st_place_text: Query<&mut Text, (With<LeaderboardScore1stPlaceText>, Without<LeaderboardName1stPlaceText>)>,
// mut leaderboard_score_2nd_place_text: Query<&mut Text, (With<LeaderboardScore2ndPlaceText>, Without<LeaderboardName2ndPlaceText>)>,
// mut leaderboard_score_3rd_place_text: Query<&mut Text, (With<LeaderboardScore3rdPlaceText>, Without<LeaderboardName3rdPlaceText>)>,
// mut leaderboard_score_4th_place_text: Query<&mut Text, (With<LeaderboardScore4thPlaceText>, Without<LeaderboardName4thPlaceText>)>,
// mut leaderboard_score_5th_place_text: Query<&mut Text, first_place: Query<Entity, With<LeaderboardName1stPlaceText>>>,
// ) {

// for e in event_reader.read() {

//     info!("heard the update score event!");

//     let default_leaderboard_data = LeaderboardUpdateData {
//         your_points: 0,
//         your_leaderboard_place: 0,

//         leaderboard_name_1st_place: "--".to_string(),
//         leaderboard_name_2nd_place: "--".to_string(),
//         leaderboard_name_3rd_place: "--".to_string(),
//         leaderboard_name_4th_place: "--".to_string(),
//         leaderboard_name_5th_place: "--".to_string(),

//         leaderboard_score_1st_place: 0,
//         leaderboard_score_2nd_place: 0,
//         leaderboard_score_3rd_place: 0,
//         leaderboard_score_4th_place: 0,
//         leaderboard_score_5th_place: 0,
//     };

//     let update_leaderboard_msg_data = serde_json::from_value(e.data.clone()).unwrap_or_else(|op| {
//         info!("Failed to parse incoming websocket message: {}", op);
//         default_leaderboard_data
//     });

//     // Update your position
//     for mut text in your_position_text.iter_mut() {
//         text.sections[0].value = format!("Position: {:?}", update_leaderboard_msg_data.your_leaderboard_place.clone());
//     }

// // update the 5 name fields
// for mut text in leaderboard_name_1st_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_1st_place.clone();
// }

// for mut text in leaderboard_name_2nd_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_2nd_place.clone();
// }

// for mut text in leaderboard_name_3rd_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_3rd_place.clone();
// }

// for mut text in leaderboard_name_4th_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_4th_place.clone();
// }

// for mut text in leaderboard_name_5th_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_name_5th_place.clone();

// // update the 5 score fields
// for mut text in leaderboard_score_1st_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_1st_place.to_string();
// }

// for mut text in leaderboard_score_2nd_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_2nd_place.to_string();
// }

// for mut text in leaderboard_score_3rd_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_3rd_place.to_string();
// }

// for mut text in leaderboard_score_4th_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_4th_place.to_string();
// }

// for mut text in leaderboard_score_5th_place_text.iter_mut() {
//     text.sections[0].value = update_leaderboard_msg_data.leaderboard_score_5th_place.to_string();
// }
// }
// }
// }

struct Score {
    name: String,
    value: u32,
}

fn setup_leaderboard_table(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Sample data for the table
    let scores = vec![
        Score {
            name: "--".to_string(),
            value: 0,
        },
        Score {
            name: "--".to_string(),
            value: 0,
        },
        Score {
            name: "--".to_string(),
            value: 0,
        },
        Score {
            name: "--".to_string(),
            value: 0,
        },
        Score {
            name: "--".to_string(),
            value: 0,
        },
    ];

    // Create a parent node for the table
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute, // Absolute positioning
                row_gap: Val::Px(7.),
                // column_gap: Val::Px(50.),
                right: Val::Percent(3.),
                top: Val::Percent(3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Header Row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        flex_grow: 1.,
                        // min_width: Val::Px(400.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Leaderboard",
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                });

            // Create rows for each score
            let y_offset = 30.0; // Adjust this value for the desired spacing
            for (index, score) in scores.into_iter().enumerate() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        let leaderboard_name_text_bundle = TextBundle {
                            text: Text::from_section(
                                format!("{}    ", &score.name), // super hacky way to add right padding... 😅
                                TextStyle {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        };

                        match index {
                            0 => {
                                parent
                                    .spawn(leaderboard_name_text_bundle)
                                    .insert(LeaderboardName1stPlaceText);
                            }
                            1 => {
                                parent
                                    .spawn(leaderboard_name_text_bundle)
                                    .insert(LeaderboardName2ndPlaceText);
                            }
                            2 => {
                                parent
                                    .spawn(leaderboard_name_text_bundle)
                                    .insert(LeaderboardName3rdPlaceText);
                            }
                            3 => {
                                parent
                                    .spawn(leaderboard_name_text_bundle)
                                    .insert(LeaderboardName4thPlaceText);
                            }
                            4 => {
                                parent
                                    .spawn(leaderboard_name_text_bundle)
                                    .insert(LeaderboardName5thPlaceText);
                            }
                            _ => (),
                        }

                        let leaderboard_score_text_bundle = TextBundle {
                            text: Text::from_section(
                                "".to_string(),
                                TextStyle {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        };

                        match index {
                            0 => {
                                parent
                                    .spawn(leaderboard_score_text_bundle)
                                    .insert(LeaderboardScore1stPlaceText);
                            }
                            1 => {
                                parent
                                    .spawn(leaderboard_score_text_bundle)
                                    .insert(LeaderboardScore2ndPlaceText);
                            }
                            2 => {
                                parent
                                    .spawn(leaderboard_score_text_bundle)
                                    .insert(LeaderboardScore3rdPlaceText);
                            }
                            3 => {
                                parent
                                    .spawn(leaderboard_score_text_bundle)
                                    .insert(LeaderboardScore4thPlaceText);
                            }
                            4 => {
                                parent
                                    .spawn(leaderboard_score_text_bundle)
                                    .insert(LeaderboardScore5thPlaceText);
                            }
                            _ => (),
                        }
                    });
            }
        });
}
