use bevy::prelude::*;

use crate::demo::movement::{MAX_X_POS, MIN_X_POS, MIN_Y_POS};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_background);
}

fn create_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let texture_handle = asset_server.load("images/ducks-bg-test.png");
    let bg_grass_img_handle = asset_server.load("v2-images/bg-grass.jpg");
    let rocks_and_spikes_top_img_handle = asset_server.load("v2-images/rocks-and-spikes.png");
    let trees_left_img_handle = asset_server.load("v2-images/trees-left.png");
    let trees_right_img_handle = asset_server.load("v2-images/trees-right.png");
    let trees_bottom_img_handle = asset_server.load("v2-images/trees-bottom.png");

    println!("creating background...");

    // Spawn an entity with the background image as a sprite
    commands.spawn(SpriteBundle {
        texture: bg_grass_img_handle,
        transform: Transform {
            scale: Vec3::new(1.0, 1.0, 0.0),
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn rocks above walkable area
    commands.spawn(SpriteBundle {
        texture: rocks_and_spikes_top_img_handle,
        transform: Transform {
            scale: Vec3::new(1., 1., 0.0),
            translation: Vec3::new(-105.,1305.0, 2.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn trees left
    commands.spawn(SpriteBundle {
        texture: trees_left_img_handle,
        transform: Transform {
            scale: Vec3::new(1., 1., 0.0),
            translation: Vec3::new(MIN_X_POS - 660.,-180.0, 100.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn trees right
    commands.spawn(SpriteBundle {
        texture: trees_right_img_handle,
        transform: Transform {
            scale: Vec3::new(1., 1., 0.0),
            translation: Vec3::new(MAX_X_POS + 380.,-150.0, 100.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn trees bottom in front of EVERYTHING (TODO - refactor a more organized layering system based on y position)
    commands.spawn(SpriteBundle {
        texture: trees_bottom_img_handle,
        transform: Transform {
            scale: Vec3::new(1., 1., 0.0),
            translation: Vec3::new(MIN_X_POS + 350.,MIN_Y_POS - 400., 200.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // commands.spawn(SpriteBundle {
    //     texture: bg_texture_handle,
    //     transform: Transform {
    //         scale: Vec3::new(1.0, 1.0, 0.0),
    //         translation: Vec3::new(0.0, 0.0, 100.0),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
}
