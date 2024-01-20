use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    AssetsLoaded,
}

#[derive(AssetCollection, Resource)]
struct SceneAssets {
    #[asset(path = "A.glb")]
    scene_a: Handle<Scene>,
    #[asset(path = "B.glb")]
    scene_b: Handle<Scene>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::AssetsLoaded)
                .load_collection::<SceneAssets>(),
        )
        .add_systems(OnEnter(GameState::AssetsLoaded), spawn_scenes)
        .run();
}

// A should be spawned to the left of B.
// Roughly 25% of the time it's actually the other way around.
// Appending #Scene0 to the end of the gltf paths fixes this.
fn spawn_scenes(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        Name::from("Scene A"),
        SceneBundle {
            scene: scene_assets.scene_a.clone(),
            transform: Transform::from_xyz(-1.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Name::from("Scene B"),
        SceneBundle {
            scene: scene_assets.scene_b.clone(),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 1.0, 5.0),
        ..default()
    });
}
