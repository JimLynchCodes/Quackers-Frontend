use bevy::color::palettes::css::{LIME, RED};
use bevy::prelude::*;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use virtual_joystick::{
    create_joystick, JoystickFloating, JoystickInvisible, NoAction, VirtualJoystickEvent,
    VirtualJoystickPlugin,
};
// use bevy::ui::{UiPlugin};
// use bevy::prelude::*;

use crate::theme::palette::{BUTTON_TEXT, NODE_BACKGROUND};
use crate::theme::widgets::Containers;

use crate::demo::other_player::{unpack_duck_color, NewJoinerDataWithAllPlayers};
use crate::{
    asset_tracking::LoadResource,
    demo::{movement::MovementController, player_animation::PlayerAnimation},
    screens::Screen,
};

use super::websocket_connect::{
    MoveCrackersBevyEvent, OtherPlayerJoinedWsReceived, YouJoinedWsReceived,
};
use super::websocket_quack_msg::QuackRequestEvent;

#[derive(Resource)]
pub struct QuackAudio {
    pub sound_handle: Handle<AudioSource>,
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
    app.register_type::<Player>();

    app.add_plugins(VirtualJoystickPlugin::<String>::default());
    // app.add_plugins(ShapePlugin);
    app.add_systems(Startup, create_joystick_scene);
    app.add_systems(Update, handle_joystick_or_keyboard_input);
    app.add_systems(Startup, quack_sound_setup);
    app.add_systems(Startup, add_quack_button);
    app.add_systems(Update, spacebar_quack_system);
    app.add_systems(Update, you_joined_ws_msg_handler);
    app.add_systems(Update, quack_btn_handler);
    app.add_systems(Startup, spacial_listener_setup);
}

fn spacial_listener_setup(
    mut commands: Commands,
    // interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    // audio: Res<QuackAudio>,
    // audio_assets: Res<Assets<AudioSource>>,
) {
}

fn quack_btn_handler(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction, &QuackBtnButton), Changed<Interaction>>,
    audio: Res<QuackAudio>,
    audio_assets: Res<Assets<AudioSource>>,
    mut quack_request_bevy_event_writer: EventWriter<QuackRequestEvent>
) {
    for (_entity, interaction, _quack_btn) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            println!("clicked quack btn!");

            if let Some(_) = audio_assets.get(&audio.sound_handle) {
                // Spawn an audio source to play the sound
                commands.spawn(AudioSourceBundle {
                    source: audio.sound_handle.clone(), // Clone the handle to use it
                    ..Default::default()                // Use default values for other fields
                });
                println!("Playing YouGotCrackers sound.");

                quack_request_bevy_event_writer.send(QuackRequestEvent);
            } else {
                println!("Audio not loaded yet.");
            }
        }
    }
}

fn add_quack_button(mut commands: Commands) {
    // Spawn a button at the bottom-right of the window
    // commands.ui_root()
    // .insert(StateScoped(Screen::Gameplay))
    //     .with_children(|children| {

    //         // children.
    //         // children.quack_button("Q").observe(enter_gameplay_screen);
    //     });


        commands.spawn(
            
            // ShapeBundle {
            //     shape: Shape::RoundedRectangle {
            //         width: 200.0,
            //         height: 100.0,
            //         radius: 20.0, // Radius for rounded corners
            //     },
            //     style: Style {
            //         position_type: PositionType::Absolute,
            //         position: Rect {
            //             left: Val::Px(100.0),
            //             top: Val::Px(100.0),
            //             ..Default::default()
            //         },
            //         ..Default::default()
            //     },
            //     draw: Draw {
            //         fill: Fill {
            //             color: Color::rgb(0.5, 0.3, 0.7), // Color of the button
            //         },
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // }
            
            
            ButtonBundle {
            // name: Name::new("QuackButton"),
            style: Style {
                width: Val::Px(65.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute, // locks button to the "HUD"
                right: Val::Percent(5.0),
                bottom: Val::Percent(5.0),
                ..Default::default()
            },
            // background_color: Color::srgb(0.15, 0.15, 0.15).into(),
            background_color: NODE_BACKGROUND.into(),
            ..Default::default()
        }
    
    )
        .insert(QuackBtnButton)
        .with_children(|children| {
            children.spawn((
                Name::new("QuackButton Text"),
                TextBundle::from_section(
                    "Q",
                    TextStyle {
                        font_size: 40.0,
                        color: BUTTON_TEXT,
                        ..default()
                    },
                ),
            ));
        });
}

// Tag for the button that follows the camera
#[derive(Component)]
struct QuackBtnButton;

fn quack_sound_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sound_handle = asset_server.load("audio/sound_effects/duck-quack.wav");

    // Store the handle in a resource
    commands.insert_resource(QuackAudio { sound_handle });
}

fn spacebar_quack_system(
    mut commands: Commands,
    audio: Res<QuackAudio>,
    keyboard_input: Res<ButtonInput<KeyCode>>, // Input resource for key events
    audio_assets: Res<Assets<AudioSource>>,    // Query to find entities to affect
    mut quack_request_bevy_event_writer: EventWriter<QuackRequestEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Space pressed!");

        if let Some(_) = audio_assets.get(&audio.sound_handle) {
            // Spawn an audio source to play the sound
            commands.spawn(AudioSourceBundle {
                source: audio.sound_handle.clone(),
                ..Default::default()
            });
            println!("Playing your quack sound.");

            quack_request_bevy_event_writer.send(QuackRequestEvent);
        } else {
            println!("Audio not loaded yet.");
        }
    }
}

fn create_joystick_scene(mut cmd: Commands, asset_server: Res<AssetServer>) {
    create_joystick(
        &mut cmd,
        "UniqueJoystick".to_string(),
        asset_server.load("Knob.png"),
        asset_server.load("Outline.png"),
        None,
        None,
        None,
        Vec2::new(75., 75.),
        Vec2::new(150., 150.),
        Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            position_type: PositionType::Absolute,
            left: Val::Percent(0.),
            bottom: Val::Percent(0.),
            ..default()
        },
        (JoystickInvisible, JoystickFloating),
        NoAction,
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

// spawn player
pub fn you_joined_ws_msg_handler(
    mut event_reader: EventReader<YouJoinedWsReceived>,
    mut bevy_move_crackers_event_writer: EventWriter<MoveCrackersBevyEvent>,
    mut commands: Commands,
    player_assets_op: Option<Res<PlayerAssets>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut bevy_event_writer_other_player_joined: EventWriter<OtherPlayerJoinedWsReceived>,
) {
    if let Some(player_assets) = player_assets_op {
        for e in event_reader.read() {
            let you_joined_response_data =
                serde_json::from_value(e.data.clone()).unwrap_or_else(|op| {
                    info!("Failed to parse incoming websocket message: {}", op);
                    NewJoinerDataWithAllPlayers {
                        player_uuid: "error".to_string(),
                        player_friendly_name: "error".to_string(),
                        color: "error".to_string(),
                        x_position: 0.,
                        y_position: 0.,
                        // player_uuid: sender_game_data.client_id.clone(),
                        // player_friendly_name: sender_game_data.friendly_name.clone(),
                        // color: sender_game_data.color.clone(),
                        // x_position: sender_game_data.x_pos,
                        // y_position: sender_game_data.x_pos,
                        cracker_x: 0.,
                        cracker_y: 0.,
                        cracker_points: 0,
                        player_points: 0,
                        all_other_players: vec![],
                    }
                });

            // play sound effect

            bevy_move_crackers_event_writer.send(MoveCrackersBevyEvent {
                x_position: you_joined_response_data.cracker_x,
                y_position: you_joined_response_data.cracker_y,
                points: you_joined_response_data.cracker_points,
                you_got_crackers: false,
            });

            info!("In player.rs handling the You joined event {:?}!", e);
            let layout =
                TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let player_animation = PlayerAnimation::new();

            // Spawn your duck!
            commands
                .spawn((
                    Name::new("foo".to_string()),
                    Player,
                    SpriteBundle {
                        texture: player_assets.ducky.clone(),
                        transform: Transform {
                            scale: Vec2::splat(4.0).extend(2.0),
                            translation: Vec3::new(
                                you_joined_response_data.x_position,
                                you_joined_response_data.y_position,
                                10.0,
                            ),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: unpack_duck_color(you_joined_response_data.color),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: player_animation.get_atlas_index(),
                    },
                    MovementController {
                        max_speed: 500.,
                        ..default()
                    },
                    player_animation,
                    StateScoped(Screen::Gameplay),
                ))
                .with_children(|parent| {
                    // Text that appears above the sprite
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            you_joined_response_data.player_friendly_name, // The text to display
                            TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"), // Load your font here
                                font_size: 25.0,
                                color: Color::WHITE,
                            },
                        ),
                        transform: Transform {
                            translation: Vec3::new(0.0, 17.0, 10.0), // Position the text above the sprite
                            scale: Vec3::splat(0.25),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                })
                .with_children(|parent| {
                    let gap = 40.0;

                    println!("Adding spatial listener!");

                    let listener = SpatialListener::new(gap);
                    parent
                        .spawn((SpatialBundle::default(), listener.clone()))
                        .with_children(|parent| {

                            // Display for debugging purposes

                            // left ear
                            // parent.spawn(SpriteBundle {
                            //     sprite: Sprite {
                            //         color: RED.into(),
                            //         custom_size: Some(Vec2::splat(20.0)),
                            //         ..default()
                            //     },
                            //     transform: Transform::from_xyz(-gap, 0.0, 100.0),
                            //     ..default()
                            // });

                            // right ear
                            // parent.spawn(SpriteBundle {
                            //     sprite: Sprite {
                            //         color: LIME.into(),
                            //         custom_size: Some(Vec2::splat(20.0)),
                            //         ..default()
                            //     },
                            //     transform: Transform::from_xyz(gap, 0.0, 100.0),
                            //     ..default()
                            // });
                        });
                });

            // Send bevy event to show other players
            for other_player_data in you_joined_response_data.all_other_players.iter() {
                bevy_event_writer_other_player_joined.send(OtherPlayerJoinedWsReceived {
                    data: other_player_data.clone(),
                });
            }
        }
    }
}

fn handle_joystick_or_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut joystick: EventReader<VirtualJoystickEvent<String>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    for j in joystick.read() {
        let Vec2 { x, y } = j.axis();

        // Joystick overrides the arrow keys! (but don't override if not touching joystick)
        if *x != 0. || *y != 0. {
            intent.x = *x;
            intent.y = *y;
        }
    }

    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct PlayerAssets {
    #[dependency]
    pub ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl PlayerAssets {
    pub const PATH_DUCKY: &'static str = "images/ducky.png";
    pub const PATH_STEP_1: &'static str = "audio/sound_effects/step1.wav";
    pub const PATH_STEP_2: &'static str = "audio/sound_effects/step2.wav";
    pub const PATH_STEP_3: &'static str = "audio/sound_effects/step3.wav";
    pub const PATH_STEP_4: &'static str = "audio/sound_effects/step4.wav";
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                PlayerAssets::PATH_DUCKY,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load(PlayerAssets::PATH_STEP_1),
                assets.load(PlayerAssets::PATH_STEP_2),
                assets.load(PlayerAssets::PATH_STEP_3),
                assets.load(PlayerAssets::PATH_STEP_4),
            ],
        }
    }
}
