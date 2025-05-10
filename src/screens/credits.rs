use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::Music, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn_credits_screen);

    app.load_resource::<CreditsMusic>();
    app.add_systems(OnEnter(Screen::Credits), play_credits_music);
    app.add_systems(OnExit(Screen::Credits), stop_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|parent| {
            // Modal container with white background and rounded corners
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        height: Val::Px(400.0),
                        padding: UiRect::all(Val::Px(30.0)), // Padding inside the panel
                        margin: UiRect::all(Val::Auto), // Centered
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(1.0, 1.0, 1.0).into(),
                    ..default()
                })
                .insert(UiImage::default())
                .insert(BackgroundColor(Color::rgb(1.0, 1.0, 1.0)))
                .insert(BorderRadius::all(Val::Px(16.0)))
                .with_children(|modal| {
                    modal.header("Credits");

                    // Centered black label
                    modal
                        .spawn(TextBundle {
                            text: Text::from_section(
                                "Jim Lynch - @JimLynchCodes",
                                TextStyle {
                                    font_size: 24.0,
                                    color: Color::rgb(0.1, 0.1, 0.1),
                                    ..default()
                                },
                            ),
                            style: Style {
                                margin: UiRect::all(Val::Auto),
                                align_self: AlignSelf::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        });

                    modal
                        .button("Back")
                        .observe(enter_title_screen);
                });
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct CreditsMusic {
    #[dependency]
    music: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for CreditsMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Monkeys Spinning Monkeys.ogg"),
            entity: None,
        }
    }
}

fn play_credits_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.music.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
            ))
            .id(),
    );
}

fn stop_music(mut commands: Commands, mut music: ResMut<CreditsMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}