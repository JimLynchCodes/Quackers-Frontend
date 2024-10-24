// "Other" player refers to all players that are not the one being controlled by the user

use bevy::{
    audio::{AudioPlugin, SpatialScale},
    color::palettes::css::*,
    prelude::*,
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    sprite::MaterialMesh2dBundle,
};
use serde::Deserialize;

use super::{
    player_animation::PlayerAnimationState,
    websocket_connect::{
        OtherPlayerJoinedWsReceived, OtherPlayerMovedWsReceived, OtherPlayerQuackedWsReceived,
        S2CActionTypes, UserDisconnectedBevyEvent,
    },
};

use crate::{
    asset_tracking::LoadResource,
    demo::other_player_animation::{OtherPlayerAnimation, OtherPlayerAnimationState},
    screens::Screen,
};

// impl PlayerAnimation {
//     pub fn play_walking(&mut self) {
//         // Assuming you have a way to set the current animation state
//         // You might have an enum or some other structure to track the state.
//         // For example, let's assume `current_animation` is a field that keeps track of the active animation.

//         self.current_animation = PlayerAnimationState::Walking; // or however you represent it

//         // Optionally, reset frame to start the walking animation from the beginning
//         self.current_frame = 0;
//     }
// }

use bevy_kira_audio::*;

// #[derive(Debug, Deserialize)]
// pub struct NewJoinerData {
//     pub player_uuid: String,
//     pub player_friendly_name: String,
//     pub color: String,
//     pub x_position: f32,
//     pub y_position: f32,
//     pub cracker_x: f32,
//     pub cracker_y: f32,
//     pub cracker_points: u64,

//     pub direction_facing: DuckDirection,

//     pub player_points: u64,
// }

// #[derive(Debug, Deserialize)]
// pub struct NewJoinerDataWithAllPlayers {
//     pub player_uuid: String,
//     pub player_friendly_name: String,
//     pub color: String,
//     pub x_position: f32,
//     pub y_position: f32,
//     pub cracker_x: f32,
//     pub cracker_y: f32,
//     pub cracker_points: u64,

//     pub player_points: u64,

//     pub all_other_players: Vec<ClientGameData>,
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct ClientGameData {
//     pub client_id: String,
//     pub x_pos: f32,
//     pub y_pos: f32,
//     pub radius: u64,

//     pub friendly_name: String,
//     pub color: String,
//     pub quack_pitch: f32,

//     pub cracker_count: u64,
//     pub leaderboard_position: u64
// }

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum DuckDirection {
    Left,
    Right,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OtherPlayerData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub x_position: f32,
    pub y_position: f32,

    pub direction_facing: DuckDirection,
}

#[derive(Debug, Deserialize)]
pub struct NewJoinerDataWithAllPlayers {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub x_position: f32,
    pub y_position: f32,
    pub cracker_x: f32,
    pub cracker_y: f32,
    pub cracker_points: u64,

    pub player_points: u64,

    pub all_other_players: Vec<OtherPlayerData>,
}

#[derive(Debug, Deserialize)]
pub struct MoveRequestData {
    pub x_direction: f32,
    pub y_direction: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MoveResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub old_x_position: f32,
    pub old_y_position: f32,
    pub new_x_position: f32,
    pub new_y_position: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserDisconnectedData {
    pub disconnected_player_uuid: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuackResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub player_x_position: f32,
    pub player_y_position: f32,
    pub quack_pitch: f32,
}

// Tag for the listener (e.g., player or camera)
#[derive(Component)]
struct Listener;

// Tag for sound emitters
// #[derive(Component)]
// struct SoundEmitter;

#[derive(Debug, Deserialize)]
pub struct OtherMovedReceivedWsMsg {
    pub action_type: S2CActionTypes,
    pub data: MoveResponseData,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct OtherPlayer;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct OtherPlayerAssets {
    #[dependency]
    pub ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<bevy::audio::AudioSource>>,
}

impl OtherPlayerAssets {
    pub const PATH_DUCKY: &'static str = "images/ducky.png";
    pub const PATH_STEP_1: &'static str = "audio/sound_effects/step1.ogg";
    pub const PATH_STEP_2: &'static str = "audio/sound_effects/step2.ogg";
    pub const PATH_STEP_3: &'static str = "audio/sound_effects/step3.ogg";
    pub const PATH_STEP_4: &'static str = "audio/sound_effects/step4.ogg";
}

impl FromWorld for OtherPlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                OtherPlayerAssets::PATH_DUCKY,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load(OtherPlayerAssets::PATH_STEP_1),
                assets.load(OtherPlayerAssets::PATH_STEP_2),
                assets.load(OtherPlayerAssets::PATH_STEP_3),
                assets.load(OtherPlayerAssets::PATH_STEP_4),
            ],
        }
    }
}

// #[derive(Resource)]
// pub struct SpatialAudio {
//     /// The volume will change from `1` at distance `0` to `0` at distance `max_distance`
//     pub max_distance: f32,
// }

// impl SpatialAudio {
//     pub(crate) fn update(
//         &self,
//         receiver_transform: &GlobalTransform,
//         emitters: &Query<(&GlobalTransform, &AudioEmitter)>,
//         audio_instances: &mut Assets<AudioInstance>,
//     ) {
//         for (emitter_transform, emitter) in emitters {
//             let sound_path = emitter_transform.translation() - receiver_transform.translation();
//             let volume = (1. - sound_path.length() / self.max_distance)
//                 .clamp(0., 1.)
//                 .powi(2);

//             let right_ear_angle = receiver_transform.right().angle_between(sound_path);
//             let panning = (right_ear_angle.cos() + 1.) / 2.;

//             for instance in emitter.instances.iter() {
//                 if let Some(instance) = audio_instances.get_mut(instance) {
//                     instance.set_volume(volume as f64, AudioTween::default());
//                     instance.set_panning(panning as f64, AudioTween::default());
//                 }
//             }
//         }
//     }
// }

// TODO - Still need to figure out proper spatial audio...
// pub(crate) fn run_spatial_audio(
//     spatial_audio: Res<SpatialAudio>,
//     receiver: Query<&GlobalTransform, With<AudioReceiver>>,
//     emitters: Query<(&GlobalTransform, &AudioEmitter)>,
//     mut audio_instances: ResMut<Assets<AudioInstance>>,
// ) {
//     if let Ok(receiver_transform) = receiver.get_single() {
//         spatial_audio.update(receiver_transform, &emitters, &mut audio_instances);
//     }
// }

// pub(crate) fn cleanup_stopped_spatial_instances(
//     mut emitters: Query<&mut AudioEmitter>,
//     instances: ResMut<Assets<AudioInstance>>,
// ) {
//     for mut emitter in emitters.iter_mut() {
//         let handles = &mut emitter.instances;

//         handles.retain(|handle| {
//             if let Some(_instance) = instances.get(handle) {
//                 _instance.handle.state() != PlaybackState::Stopped
//             } else {
//                 true
//             }
//         });
//     }
// }

/// Component for audio emitters
///
/// Add [`Handle<AudioInstance>`]s to control their pan and volume based on emitter
/// and receiver positions.
// #[derive(Component, Default)]
// pub struct AudioEmitter {
//     /// Audio instances that are played by this emitter

//     /// The same instance should only be on one emitter.
//     pub instances: Vec<Handle<AudioInstance>>,
// }

/// Component for the audio receiver
///
/// Most likely you will want to add this component to your player or you camera.
/// The entity needs a [`Transform`] and [`GlobalTransform`]. The view direction of the [`GlobalTransform`]
/// will
// #[derive(Component)]
// pub struct AudioReceiver;

// #[derive(Bundle)]
// struct SpatialAudioBundle {
//     audio_source: Handle<bevy_kira_audio::AudioSource>,
//     transform: Transform,
//     global_transform: GlobalTransform,
// }

/// Spatial audio uses the distance to attenuate the sound volume. In 2D with the default camera,
/// 1 pixel is 1 unit of distance, so we use a scale so that 100 pixels is 1 unit of distance for
/// audio.

// const AUDIO_SCALE: f32 = 1. / 100.0;

#[derive(Component, Default)]
pub struct Emitter {
    stopped: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<OtherPlayer>();
    app.load_resource::<OtherPlayerAssets>();
    app.add_plugins(bevy_kira_audio::AudioPlugin);
    // app.add_plugins(DefaultPlugins.set(AudioPlugin {
    //     default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
    //     ..default()
    // }));
    // app.insert_resource(SpatialAudio { max_distance: 25. });
    // app.init_asset::<AudioSource>();
    // app.insert_resource(SpatialScale { scale: 1.0 }); // Scale of spatial audio

    app.add_systems(Update, other_player_joined_ws_msg_handler);
    app.add_systems(Update, other_player_moved_ws_msg_handler);
    app.add_systems(Update, other_player_quacked_handler);
    app.add_systems(Update, other_player_disconnected_handler);

    // app.add_systems(
    //     Update,
    //     run_spatial_audio.run_if(resource_exists::<SpatialAudio>),
    // );
}

// app.add_plugins(DefaultPlugins.set(AudioPlugin {
//     default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
//     ..default()
// }));

// spawn player
pub fn other_player_joined_ws_msg_handler(
    mut event_reader: EventReader<OtherPlayerJoinedWsReceived>,
    mut commands: Commands,
    player_assets_op: Option<Res<OtherPlayerAssets>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Some(player_assets) = player_assets_op {
        for e in event_reader.read() {
            info!("other player joined!");

            // #[derive(Debug, Deserialize)]
            // pub struct NewJoinerData {
            //     pub player_uuid: String,
            //     pub player_friendly_name: String,
            //     pub color: String,
            //     pub x_position: f32,
            //     pub y_position: f32,
            // }

           

            info!(
                "In other_player.rs handling the Other Player joined event {:?}!",
                e
            );
            let layout =
                TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let player_animation = OtherPlayerAnimation::new();

            let parent_entity = (
                Name::new(e.data.player_uuid.clone()),
                OtherPlayer,
                SpriteBundle {
                    texture: player_assets.ducky.clone(),
                    transform: Transform {
                        scale: Vec3::new(
                            // 4.0 * if e.data.direction_facing
                            //     == DuckDirection::Left
                            // {
                            //     -1.
                            // } else {
                            //     1.
                            // },
                            4.0,
                            4.0,
                            2.0,
                        ),
                        translation: Vec3::new(
                            e.data.x_position,
                            e.data.y_position,
                            10.0,
                        ),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: unpack_duck_color(e.data.color.clone()),
                        flip_x: e.data.direction_facing == DuckDirection::Left,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: player_animation.get_atlas_index(),
                },
                player_animation,
                StateScoped(Screen::Gameplay),
            );

            commands.spawn(parent_entity).with_children(|parent| {
                // Player name text that appears above the sprite
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        e.data.player_friendly_name.clone(), // The text to display
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"), // Load your font here
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::new(0.0, 17.0, 1.0), // Position the text above the sprite
                        scale: Vec3::new(
                        //     0.25 * if e.data.direction_facing // re-flip text so it's always readable
                        //     == DuckDirection::Left
                        // {
                        //     -1.
                        // } else {
                        //     1.
                        // }, 0.25, 1.0),
                        0.25, 0.25, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        }
    }
}

// spawn player
pub fn other_player_moved_ws_msg_handler(
    mut event_reader: EventReader<OtherPlayerMovedWsReceived>,
    mut other_players: Query<(
        &OtherPlayer,
        &mut Sprite,
        &Name,
        &mut Transform,
        &mut OtherPlayerAnimation,
    )>, // friendly_name: String,
        // color: Color,
        // max_speed: f32,
) {
    for e in event_reader.read() {
        info!("Handling other player moved bevy event");

        let other_player_moved_response_data = serde_json::from_value(e.data.clone())
            .unwrap_or_else(|op| {
                info!("Failed to parse incoming websocket message: {}", op);
                MoveResponseData {
                    player_uuid: "error".to_string(),
                    player_friendly_name: "error".to_string(),
                    color: "error".to_string(),
                    old_x_position: 0.,
                    old_y_position: 0.,
                    new_x_position: 0.,
                    new_y_position: 0.,
                }
            });

        info!(
            "In other_player.rs handling the Other Player moved event {:?}!",
            e
        );

        for (_other_player, mut sprite, name, mut transform, mut animation) in
            other_players.iter_mut()
        {
            info!("checking vs id map: {}", name.to_string());
            if name.to_string() == other_player_moved_response_data.player_uuid {
                println!(
                    "Found entity with id: {}",
                    other_player_moved_response_data.player_uuid
                );

                transform.translation.x = other_player_moved_response_data.new_x_position;
                transform.translation.y = other_player_moved_response_data.new_y_position;

                let dx = other_player_moved_response_data.new_x_position
                    - other_player_moved_response_data.old_x_position;

                sprite.flip_x = dx < 0.;

                // animation.play_walking();

                animation.update_state(OtherPlayerAnimationState::Walking);
            }
        }
    }
}

pub fn unpack_duck_color(color: String) -> Color {
    match color.as_str() {
        "white" => Color::WHITE,
        "teal" => Color::srgba(0.8, 1.0, 1.0, 1.0), // Teal
        "yellow" => Color::srgba(1.0, 1.0, 0.8, 1.0), // Yellow
        "purple" => Color::srgba(0.70, 0.6, 1.0, 1.0), // Purple
        "pink" => Color::srgba(1.0, 0.84, 0.87, 1.0), // Pink
        "light_orange" => Color::srgba(1.0, 0.8, 0.1, 1.0), // Light orange
        "baby_blue" => Color::srgba(0.54, 0.81, 0.94, 1.), // Baby blue
        "lime_green" => Color::srgba(0.60, 1.0, 0.60, 1.), // Lime Green
        _ => Color::WHITE, // Default color

        // _ => Color::srgba(1.0, 0.84, 0.87, 1.0), // For testing colors
    }
}

fn other_player_disconnected_handler(
    mut commands: Commands,
    mut event_reader: EventReader<UserDisconnectedBevyEvent>,
    other_player_entities: Query<(Entity, &OtherPlayer, &Name)>,
) {
    for e in event_reader.read() {
        let other_player_disconnected_data =
            serde_json::from_value(e.data.clone()).unwrap_or_else(|op| {
                info!(
                    "Failed to parse incoming player disconnected websocket message: {}",
                    op
                );
                UserDisconnectedData {
                    disconnected_player_uuid: "error".to_string(),
                }
            });

        for (entity, _component, name) in other_player_entities.iter() {
            if name.to_string() == other_player_disconnected_data.disconnected_player_uuid {
                commands.entity(entity).despawn_recursive();
                println!("Deleting duck for user: {}", name);
            };
        }
    }
}

// Just plays regular quack sound for now, without spatial audio
fn other_player_quacked_handler(
    mut commands: Commands,
    mut event_reader: EventReader<OtherPlayerQuackedWsReceived>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for e in event_reader.read() {
        let other_player_quacked_response_data = serde_json::from_value(e.data.clone())
            .unwrap_or_else(|op| {
                info!("Failed to parse incoming websocket message: {}", op);
                QuackResponseData {
                    player_uuid: "error".to_string(),
                    player_friendly_name: "error".to_string(),
                    player_x_position: 0.,
                    player_y_position: 0.,
                    quack_pitch: 0.,
                }
            });

        info!(
            "Got the quack info! {:?}",
            other_player_quacked_response_data
        );

        // Get the position from the event data
        let _quack_position = Vec3::new(
            other_player_quacked_response_data.player_x_position,
            other_player_quacked_response_data.player_y_position,
            0.0, // Assuming 2D game, z-coordinate is 0
        );

        // let audio_handle = asset_server.load("audio/sound_effects/duck-quack.ogg");

        // if let Some(audio_handle) = audio_assets.get(&quack_audio.sound_handle) {
        // Spawn an audio source to play the sound
        // commands.spawn()

        // Nonspacial version
        // commands.spawn(AudioSourceBundle {
        //     source: quack_audio.sound_handle.clone(), // Clone the handle to use it

        //     ..Default::default()                // Use default values for other fields
        // });

        //  Non spatial
        // commands
        //     .spawn(TransformBundle::from_transform(Transform::from_xyz(
        //         other_player_quacked_response_data.player_x_position,
        //         other_player_quacked_response_data.player_y_position,
        //         0.0,
        //     ))) // Position emitter to the right
        //     .insert(SoundEmitter);

        // let audio_handle = asset_server.load("audio/sound_effects/duck-quack.ogg");
        // audio.play(audio_handle);

        commands.spawn((
            // MaterialMesh2dBundle {
            //     mesh: meshes.add(Circle::new(15.0)).into(),
            //     material: materials.add(Color::from(BLUE)),
            //     transform: Transform::from_translation(Vec3::new(
            //         other_player_quacked_response_data.player_x_position,
            //         other_player_quacked_response_data.player_y_position,
            //         100.0,
            //     )),
            //     ..default()
            // },
            Emitter::default(),
            AudioBundle {
                source: asset_server.load("audio/sound_effects/duck-quack.ogg"),
                settings: PlaybackSettings::DESPAWN.with_spatial(true),
            },
        ));

        // commands.spawn(SpatialAudioBundle {
        //     audio_source: audio_handle.clone(),
        //     transform: Transform::from_translation(Vec3::new(
        //         other_player_quacked_response_data.player_x_position,
        //         other_player_quacked_response_data.player_y_position,
        //         0.0,
        //     )),
        //     global_transform: GlobalTransform::default(),
        // });
        // // Play the audio at the position defined above
        // audio.play(audio_handle.clone());

        //     audio_handle,
        //     Transform {
        //         translation: Vec3::new(other_player_quacked_response_data.player_x_position,
        //             other_player_quacked_response_data.player_y_position, 0.0), // Position the audio in 3D space
        //         ..Default::default()
        //     },
        //     GlobalTransform::default(), // Required for spatial positioning
        // ));

        // commands.spawn((
        //     SpatialAudio {
        //         emitter_position: quack_position, // Position of the quacking player
        //         listener_position: listener_transform.translation, // Player's current position (listener)
        //         ..Default::default() // Other default values, such as attenuation and falloff
        //     },
        //     audio.sound_handle.clone(), // Play the quack sound
        //     PlaybackSettings::ONCE.with_pitch(other_player_quacked_response_data.quack_pitch), // Adjust the pitch dynamically
        // ));

        //     println!("Received quack message! Playing sound.");
        // } else {
        //     println!("Audio not loaded yet.");
        // }
    }
}
