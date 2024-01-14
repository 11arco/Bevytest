use bevy::prelude::*;
use bevy::window::PrimaryWindow;

//use bevy_asset::{AssetServer, Handle};
//use bevy_ecs::prelude::{Commands, Res};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
    let charright = asset_server.load("character.png");
    let texture: Handle<Image> = charright ;
     
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50.0, 50.0)), //otherwise sprite is a 1px white "dot"
            ..default()
        },
        texture,
        ..default()
    });
}

fn main(){
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Something".into(),
                    resolution: (1920.0, 1080.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_button_input)
        .add_systems(Update, character_movement)
        .run();
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    destination: Query<&Window, With<PrimaryWindow>>,

) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 50.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 200.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 200.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 200.0 * time.delta_seconds();
        }
    }
}

fn mouse_button_input(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.just_released(MouseButton::Right) {
        // right
        cursor_position(q_windows);
    }
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}