use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::sprite::Anchor;
use bevy::window::WindowMode;
use bevy::time::Time;

#[allow(unused)]
// ウィンドウ設定
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
// ゲームの背景色
const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const PRESS_ANY_KEY_FONT_SIZE: f32 = 50.0;
const PRESS_ANY_KEY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

const SLIDER_SIZE: Vec2 = Vec2::new(500.0, 50.0);
const SLIDER_DEFAULT_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const SLIDER_DEFAULT_POINTS: isize = -100;

// スコアボードの文字表示
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_COLOR: Color = Color::BLACK;


const SLIDER_OK_RANGE: f32 = 100.0;
const SLIDER_OK_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const SLIDER_OK_POINTS: isize = 10;

const SLIDER_GOOD_RANGE: f32 = 60.0;
const SLIDER_GOOD_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
const SLIDER_GOOD_POINTS: isize = 50;

const SLIDER_PERFECT_RANGE: f32 = 20.0;
const SLIDER_PERFECT_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const SLIDER_PERFECT_POINTS: isize = 100;

// キュー
const CUE_SIZE: Vec2 = Vec2::new(5.0, 50.0);
const CUE_SPEED: f32 = 500.0;
const INITIAL_CUE_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const CUE_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);


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
struct Cue;

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
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, decide_timing.run_if(in_state(AppState::PlayingGame)))
        .add_systems(Update, apply_velocity.run_if(in_state(AppState::PlayingGame)))
        // .add_systems(Update, update_scoreboard.run_if(in_state(AppState::PlayingGame)))
        .run();
}

///
/// タイミングゲームのセットアップ
/// 必要な bundle を生成する
///
fn setup (mut commands: Commands, asset_server: Res<AssetServer>) {
    // カメラ（画面描画用）を生成
    commands.spawn(Camera2d::default());

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
        PressAnyKey,
    ));

    // スライダーの生成
    commands.spawn(
        Sprite {
            color: SLIDER_DEFAULT_COLOR,
            custom_size: Some(Vec2::new(WINDOW_SIZE.x, SLIDER_SIZE.y).into()),
            anchor: Anchor::Center,
            ..default()
        },
    );

    // OK ゾーンの生成
    [
        (SLIDER_OK_COLOR, SLIDER_OK_RANGE),
        (SLIDER_GOOD_COLOR, SLIDER_GOOD_RANGE),
        (SLIDER_PERFECT_COLOR, SLIDER_PERFECT_RANGE),
    ]
        .iter()
        .for_each(|(color, range)| {
            commands.spawn(
                Sprite {
                    color: *color,
                    custom_size: Some(Vec2::new(range * 2.0, SLIDER_SIZE.y).into()),
                    ..default()
                },
            );
        });

    // キューの生成
    commands.spawn((
        Sprite{
            color: CUE_COLOR,
            custom_size: Some(CUE_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Cue,
        Velocity(INITIAL_CUE_DIRECTION.normalize() * CUE_SPEED),
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
    mut now_state: ResMut<State<AppState>>,
    mut inkey: ResMut<ButtonInput<KeyCode>>
) {
    for _event in key_board_event.read() {
        let press_any_key_entity = press_any_key_query.single();
        commands.entity(press_any_key_entity).try_despawn();
        // commands.entity(press_any_key_entity).despawn();

        *now_state = State::new(AppState::PlayingGame);
        inkey.reset_all();
    }
}

///
/// スコアボードに表示するスコアの更新
/// AppState が PlayingGame の状態で使用
///
fn update_scoreboard (
    score_board: Res<ScoreBoard>,
    mut score_board_query: Query<&mut Text, With<ScoreBoard>>,
) {
    // スコアボードのクエリアイテムを取得する．
    // TODO: ここで落ちる
    let mut text = score_board_query.single_mut();
    // スコアを更新する
    **text = score_board.score.to_string();
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
        if trans_x >= SLIDER_SIZE.x / 2.0 || trans_x <= - SLIDER_SIZE.x / 2.0 {
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
    cue_query: Query<&Transform, With<Cue>>,
    asset_server: Res<AssetServer>,
) {
    // キューの取得
    let cue_transform = cue_query.single();

    // スペースキーの入力があれば処理を行う
    if keyboard_input.just_pressed(KeyCode::Space) {
        // キー押下イベントの通知
        commands.spawn((
            AudioPlayer::new(asset_server.load("sounds/timing.ogg")),
        ));

        //
        let cue_translation_x = cue_transform.translation.x;

        if cue_translation_x < SLIDER_PERFECT_RANGE && cue_translation_x > -SLIDER_PERFECT_RANGE {
            score_board.score += SLIDER_PERFECT_POINTS;
        } else if cue_translation_x < SLIDER_GOOD_RANGE && cue_translation_x > -SLIDER_GOOD_RANGE {
            score_board.score += SLIDER_GOOD_POINTS;
        } else if cue_translation_x < SLIDER_OK_RANGE && cue_translation_x > -SLIDER_OK_RANGE {
            score_board.score += SLIDER_OK_POINTS;
        } else {
            score_board.score += SLIDER_DEFAULT_POINTS;
        }
    }

}