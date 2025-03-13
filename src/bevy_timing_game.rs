use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::sprite::Anchor;
use bevy::window::WindowMode;
use bevy::time::Time;
use bevy::color::palettes::css;

#[allow(unused)]
// ウィンドウ設定
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
// ゲームの背景色
const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// "Press Any Key..."
const PRESS_ANY_KEY_FONT_SIZE: f32 = 50.0;
const PRESS_ANY_KEY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

// Score Board
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_COLOR: Color = Color::BLACK;

// "譜面部分" の設定
const SLIDER_SIZE: Vec2 = Vec2::new(500.0, 50.0);
const SLIDER_DEFAULT_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const SLIDER_DEFAULT_POINTS: isize = 0;

// "可"
const SLIDER_GOOD_RANGE: f32 = 40.0;
const SLIDER_GOOD_POINTS: isize = 50;
const SLIDER_GOOD_COLOR: Color = Color::Srgba(css::GRAY);

// "良"
const SLIDER_PERFECT_RANGE: f32 = 24.0;
const SLIDER_PERFECT_POINTS: isize = 100;
const SLIDER_PERFECT_COLOR: Color = Color::Srgba(css::GOLD);
const SLIDER_PERFECT_PADDING: f32 = (SLIDER_GOOD_RANGE - SLIDER_PERFECT_RANGE) / 2.0;


const SLIDER_RANGE_PADDING: f32 = 100.0;
const NOTES_OK_RANGE_OFFSET: f32 = -(WINDOW_SIZE.x / 2.0) + SLIDER_RANGE_PADDING;



// const SLIDER_OK_RANGE: f32 = 100.0;
// const SLIDER_OK_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
// const SLIDER_OK_POINTS: isize = 10;

// 音符
const NOTE_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const NOTE_SPEED: f32 = -400.0;
const NOTE_COLOR: Color = Color::Srgba(css::RED);
const INITIAL_NOTE_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);


#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    PlayingGame,
}

#[derive(Resource, Component)]
struct ScoreBoard {
    score: isize,
}

#[derive(Component)]
struct PressAnyKey;

#[derive(Component)]
struct NOTE;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

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
        .insert_resource(ScoreBoard { score: 0 })
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), setup_title_screen)
        .add_systems(OnEnter(AppState::PlayingGame), setup_play_game_screen)
        .add_systems(Update, switch_state)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, decide_timing.run_if(in_state(AppState::PlayingGame)))
        .add_systems(Update, apply_velocity.run_if(in_state(AppState::PlayingGame)))
        .add_systems(Update, update_scoreboard.run_if(in_state(AppState::PlayingGame)))
        .run();
}

///
/// タイミングゲームのセットアップ
/// 必要な bundle を生成する
///
fn setup (mut commands: Commands) {
    // カメラ（画面描画用）を生成
    commands.spawn(Camera2d::default());
}

///
/// MainMenu 遷移時のセットアップ関数
/// 必要な bundle を生成する
///
fn setup_title_screen (mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Press Any Key" の文字を表示する
    commands.spawn((
        Text::new("Press Any Key ..."),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: PRESS_ANY_KEY_FONT_SIZE,
            ..default()
        },
        TextColor(PRESS_ANY_KEY_COLOR.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        // Node {
        //     display: Display::None,
        //     position_type: PositionType::Absolute,
        //     ..default()
        // },
        PressAnyKey,
    ));
}

///
/// PlayingGame 遷移時のセットアップ関数
/// 必要な bundle を生成する
///
fn setup_play_game_screen (mut commands: Commands, asset_server: Res<AssetServer>) {
    // スライダーの生成
    commands.spawn(
        Sprite {
            color: SLIDER_DEFAULT_COLOR,
            custom_size: Some(Vec2::new(WINDOW_SIZE.x, SLIDER_SIZE.y).into()),
            anchor: Anchor::Center,
            ..default()
        },
    );

    // // デバッグ用
    // [
    //     (Color::Srgba(css::GREEN), NOTES_OK_RANGE_OFFSET, 1.0),                                                     // good_min
    //     (Color::Srgba(css::GREEN), NOTES_OK_RANGE_OFFSET + SLIDER_GOOD_RANGE, 1.0),                                 // good_max
    //     (Color::Srgba(css::RED), NOTES_OK_RANGE_OFFSET + SLIDER_PERFECT_PADDING, 1.0),                              // perfect_min
    //     (Color::Srgba(css::RED), NOTES_OK_RANGE_OFFSET + SLIDER_PERFECT_PADDING + SLIDER_PERFECT_RANGE, 1.0),       // perfect_max
    //     (Color::Srgba(css::BLACK), 0.0, 1.0),
    // ]
    //     .iter()
    //     .for_each(|(color, pos, range)| {
    //         commands.spawn((
    //             Sprite {
    //                 color: *color,
    //                 custom_size: Some(Vec2::new(range.clone(), SLIDER_SIZE.y + 20.0).into()),
    //                 ..default()
    //             },
    //             // PERFECT : -300 + (24 / 2) + 0 = -300 + 12 = -288
    //             // GOOD    : -300 + (40 / 2) + 8 = -300 + 28 = -272
    //             Transform::from_xyz(*pos, 0.0, 1.0),
    //         ));
    //     });

    // ノーツ判定場所の生成
    [
        (SLIDER_GOOD_COLOR, SLIDER_GOOD_RANGE, 1.0, 0.0),
        (SLIDER_PERFECT_COLOR, SLIDER_PERFECT_RANGE, 2.0, SLIDER_PERFECT_PADDING),
    ]
        .iter()
        .for_each(|(color, range, pos_z, padding)| {
            commands.spawn((
                Sprite {
                    color: *color,
                    custom_size: Some(Vec2::new(range.clone(), SLIDER_SIZE.y).into()),
                    ..default()
                },
                // PERFECT : -300 + (24 / 2) + 0 = -300 + 12 = -288
                // GOOD    : -300 + (40 / 2) + 8 = -300 + 28 = -272
                Transform::from_xyz(NOTES_OK_RANGE_OFFSET + (range / 2.0) + padding, 0.0, *pos_z),
            ));
        });

    // ノーツの生成
    commands.spawn((
        Sprite{
            color: NOTE_COLOR,
            custom_size: Some(NOTE_SIZE),
            ..default()
        },
        Transform::from_xyz(WINDOW_SIZE.x / 2.0 - 100.0, 0.0, 3.0),
        Velocity(INITIAL_NOTE_DIRECTION.normalize() * NOTE_SPEED),
        NOTE,
    ));

    // スコアボードの生成
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(SCOREBOARD_COLOR.into()),
        TextLayout::new_with_justify(JustifyText::Left),
    ))
    .with_child((
        TextSpan::new("0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(SCOREBOARD_COLOR.into()),
        ScoreBoard { score: 0 },
    ));
}

///
/// キー押下イベントハンドラ
/// AppState が MainMenu の状態で使用
///
fn press_any_key (
    mut key_board_event: EventReader<KeyboardInput>,
    press_any_key_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut inkey: ResMut<ButtonInput<KeyCode>>
) {
    for _event in key_board_event.read() {
        let press_any_key_entity = press_any_key_query.single();
        commands.entity(press_any_key_entity).despawn();
        inkey.reset_all();
    }
}

///
/// State の切り替え
///
fn switch_state (
    mut next_state: ResMut<NextState<AppState>>,
    key_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        if *current_state.get() == AppState::MainMenu {
                next_state.set(AppState::PlayingGame);
        }
    }
}

///
/// スコアボードに表示するスコアの更新
/// AppState が PlayingGame の状態で使用
///
fn update_scoreboard (
    score_board: Res<ScoreBoard>,
    mut score_board_query: Query<&mut TextSpan, With<ScoreBoard>>,
) {
    // スコアボードのクエリアイテムを取得する．
    let mut text_span = score_board_query.single_mut();
    // スコアを更新する
    **text_span = score_board.score.to_string();
}

///
/// 速度を適用する
///
fn apply_velocity (
    mut query: Query<(&mut Transform, &mut Velocity)>,
    time_step: Res<Time<Fixed>>
) {
    for (mut trans, mut velocity) in &mut query {
        let trans_x = trans.translation.x;
        let note_offset = NOTE_SIZE.x / 2.0;
        if trans_x >= WINDOW_SIZE.x / 2.0 - note_offset || trans_x <= - WINDOW_SIZE.x / 2.0 + note_offset {
            velocity.0 = -velocity.0;
        }
        trans.translation.x += velocity.x * time_step.delta().as_secs_f32();
    }
}

///
/// キー入力時のタイミング判定
///
fn decide_timing (
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut score_board: ResMut<ScoreBoard>,
    note_query: Query<&Transform, With<NOTE>>,
    asset_server: Res<AssetServer>,
) {
    // 音符位置の取得
    let note_transform = note_query.single();

    // スペースキーの入力があれば処理を行う
    if keyboard_input.just_pressed(KeyCode::Space) {
        // キー押下イベントの通知
        commands.spawn((
            AudioPlayer::new(asset_server.load("sounds/timing.ogg")),
        ));

        //
        let note_transform_x = note_transform.translation.x;
        let perfect_min = NOTES_OK_RANGE_OFFSET + SLIDER_PERFECT_PADDING;
        let perfect_max = NOTES_OK_RANGE_OFFSET + SLIDER_PERFECT_PADDING + SLIDER_PERFECT_RANGE;
        let good_min = NOTES_OK_RANGE_OFFSET;
        let good_max = NOTES_OK_RANGE_OFFSET + SLIDER_GOOD_RANGE;

        if perfect_min < note_transform_x && note_transform_x < perfect_max {
            score_board.score += SLIDER_PERFECT_POINTS;
        } else if good_min < note_transform_x && note_transform_x < good_max {
            score_board.score += SLIDER_GOOD_POINTS;
        } else {
            score_board.score += SLIDER_DEFAULT_POINTS;
        }
    }

}