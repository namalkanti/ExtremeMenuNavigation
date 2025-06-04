use bevy::prelude::*;
use bevy::ui::Interaction;

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
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    ..default()
                },
                ..default()
            },
            Name::new("MainMenuRoot"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                MenuButton {
                    target_state: AppState::InGame,
                },
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Play", text_style.clone()));
            });

            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                MenuButton {
                    target_state: AppState::Settings,
                },
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Settings", text_style.clone()));
            });

            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                MenuButton {
                    target_state: AppState::default(), // Stay in MainMenu, you can add exit logic later
                },
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Quit", text_style));
            });
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

fn cleanup_menu(mut commands: Commands, root_query: Query<Entity, With<Name>>) {
    for entity in &root_query {
        commands.entity(entity).despawn_recursive();
    }
}
