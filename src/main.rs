use bevy::{camera::ScalingMode, prelude::*};
use bevy::window::{Window, WindowResolution, WindowPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { //GAME WINDOW SETTINGS
                    primary_window: Some(Window {
                        title: "Toy RTS RTS-Controlling System".to_string(),
                        resolution: WindowResolution::new(1600, 900),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()) //PIXEL PERFECT 2D TEXTURES RENDERER
        )
        .add_systems(Startup, str_game_init)
        .add_systems(Update, (udt_rts_controlling_system))
        .run();
}

fn str_game_init(mut commands: Commands,asset_server: Res<AssetServer>){
    
}

fn udt_rts_controlling_system(mut commands: Commands,asset_server: Res<AssetServer>){
    
}

