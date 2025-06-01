use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, load_sprite)
    .run();

    //TODO Pre game start load all spritesheets
    //TODO figure out how to read spritesheets
}

fn load_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(Camera2d);

    commands.spawn(Sprite::from_image(
        asset_server.load("runes/PNG/Grey/Slab/runeGrey_slab_001.png"),
    ));    
}

#[derive(Component)]
struct Rune;