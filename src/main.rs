use bevy::{camera::*, prelude::*};
use bevy::window::{Window, WindowResolution, WindowPlugin};
//use bevy_cursor::prelude::*;

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
        //.add_plugins(TrackCursorPlugin)
        .add_systems(Startup, str_game_init)
        .add_systems(Update, udt_rts_controlling_system)
        .run();
}



#[derive(Component)]
struct SelectionBox{
}

fn str_game_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Segment2d::new(
        Vec2::new(-50.0, 50.0),
        Vec2::new(50.0, -50.0),
    ));
    const CIRCLE_COLOR: bevy::prelude::Color = Color::hsl(0.0, 0.0, 1.0);
    commands.spawn((
        SelectionBox,
        Mesh2d(shape),
        MeshMaterial2d(materials.add(CIRCLE_COLOR)),
        Transform::from_xyz(0., 0., 0.),
    ));
}



fn udt_rts_controlling_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>, 
    window: Single<&Window>, 
    camera_query: Single<(&Camera, &GlobalTransform)>
    mut selection_box_x_query: Query<(&mut Transform, &mut SelectionBox)>,
){
    if buttons.pressed(MouseButton::Left) {
        println!("DRAGGING");

        let (camera, camera_transform) = *camera_query;
        
        let cursor_position = window.cursor_position().unwrap();

        let cursor_world_position = camera.viewport_to_world_2d(camera_transform, cursor_position);
        
        println!("Cursor position: {cursor_world_position:?}");
    }
}

