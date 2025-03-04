use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::sprite::Anchor;
// use bevy::audio::Volume;
// use bevy::window::WindowResolution;
use bevy::window::WindowMode;
use bevy::time::Time;

#[allow(unused)]
// ウィンドウ設定
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
// ゲームの背景色
const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const PRESSANYKEY_FONT_SIZE: f32 = 50.0;
const PRESSANYKEY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

const SLIDER_SIZE: Vec2 = Vec2::new(500.0, 50.0);
const SLIDER_DEFAULT_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    PlayingGame,
}

#[derive(Component)]
struct PressAnyKey;

///
/// タイミングゲームの実行
///
#[allow(unused)]
pub fn play_game() {
    // ウィンドウ設定
    let window_plugin = WindowPlugin {
        primary_window: Some( Window {
            resolution: WINDOW_SIZE.into(),
            title: "Timing Game".into(),
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    };

    // タイミングゲームの起動
    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .init_state::<AppState>()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        // .add_systems(Update, update_scoreboard.run_if(in_state(AppState::PlayingGame)))
        .run();
}

fn setup (mut commands: Commands, asset_server: Res<AssetServer>) {
    // カメラ（画面描画用）を生成
    commands.spawn(Camera2d::default());

    // "Press Any Key" の文字を表示する
    commands.spawn((
        Text::new("Press Any Key ..."),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: PRESSANYKEY_FONT_SIZE,
            ..default()
        },
        TextColor(PRESSANYKEY_COLOR.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        PressAnyKey,
    ));

    // スライドバーの生成
    commands.spawn(
        Sprite {
            color: SLIDER_DEFAULT_COLOR,
            custom_size: Some(Vec2::new(WINDOW_SIZE.x, SLIDER_SIZE.y).into()),
            anchor: Anchor::Center,
            ..default()
        },
    );

    //
}

fn press_any_key (
    mut key_board_event: EventReader<KeyboardInput>,
    press_any_key_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut now_state: ResMut<State<AppState>>,
    mut inkey: ResMut<ButtonInput<KeyCode>>
) {
    for _event in key_board_event.read() {
        let press_any_key_entity = press_any_key_query.single();
        commands.entity(press_any_key_entity).despawn();
        *now_state = State::new(AppState::PlayingGame);
        inkey.reset_all();
    }
}

// fn update_scoreboard (
//     mut key_board_event: EventReader<KeyboardInput>,
//     press_any_key_query: Query<Entity, With<PressAnyKey>>,
//     mut commands: Commands,
//     mut now_state: ResMut<State<AppState>>,
//     mut inkey: ResMut<ButtonInput<KeyCode>>
// ) {
// }
