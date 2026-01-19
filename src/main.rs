use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*};

mod api;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    Settings,
}

#[derive(Component)]
struct MenuButton {
    target_state: AppState,
}

#[derive(Component)]
struct MainMenuRoot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(Update, handle_button_click.run_if(in_state(AppState::MainMenu)))
        .add_systems(OnExit(AppState::MainMenu), cleanup_menu)
        .run();
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Name::new("MainMenuRoot"),
            MainMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent: &mut ChildSpawnerCommands| {
            spawn_menu_button(parent, "Play", AppState::InGame, &font);
            spawn_menu_button(parent, "Settings", AppState::Settings, &font);
            spawn_menu_button(parent, "Quit", AppState::MainMenu, &font);
        });
}

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    target_state: AppState,
    font: &Handle<Font>,
) {
    parent
        .spawn((
            Button,
            MenuButton { target_state },
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        ))
        .with_children(|button: &mut ChildSpawnerCommands| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn handle_button_click(
    mut next_state: ResMut<NextState<AppState>>,
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match &menu_button.target_state {
                AppState::MainMenu => {
                    info!("Quit clicked — implement exit if needed");
                }
                state => {
                    next_state.set(state.clone());
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, root_query: Query<Entity, With<MainMenuRoot>>) {
    for entity in &root_query {
        commands.entity(entity).despawn();
    }
}
