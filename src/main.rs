mod variables;
use variables::*;

use image::{Rgb, RgbImage};
use std::collections::HashMap;
use std::env;
use std::process;
use study_rust::Config;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const AMOUNT_OF_MOVEMENT: i64 = 40;
const SAVE_FILE: &str = "image.png";

///
/// main 関数
///
fn main() {
    if variables::variables() {
        println!("Called variables()\n");
    }
    run_draw_fern();
}

///
/// ファイルを探すプログラム
///
fn search_file() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else( |err| {
        eprintln!("Problem : {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = study_rust::run(config) {

        println!("Application error : {}", e);
        process::exit(1);
    }
}

///
/// シダ植物の描画
///
fn run_draw_fern() {
    // シダの描画
    // 画像バッファの作成
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // 描画
    draw_fern(&mut img, 23, 0.0, 0.0);

    // ファイルに保存する
    image::save_buffer(SAVE_FILE, &img, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}

///
/// フィボナッチ数列
///
fn run_fib() {
    // 整数
    let n_count: i32 = 10;

    // 計算結果を保持する配列c
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    // フィボナッチ数列の計算
    let n_result: i32 = calc_fib(10, &mut hash_map);
    println!("calc_fibo {} is {}.", n_count, n_result);
}

///
/// シーザー暗号化の実行
///
fn run_caesar() {
    // シーザー暗号
    let tgt_str = "Hello World!";
    println!("{}", tgt_str);
    let enc_str = caesar_rotate(&tgt_str, 3);
    println!("{}", enc_str);
    let dec_str = caesar_rotate(&enc_str, -3);
    println!("{}", dec_str);

    let result_str: String = run_caesar_encrypt(tgt_str, 3);
    println!("{}", result_str);
}

///
/// 文字列の暗号化（シーザー暗号）
/// ### Arguments
/// * text : &str        暗号化したい文字列
/// * shift : i32        文字列をシフトする数
/// ### Return
/// * enc_str : String   文字列の暗号化
///
fn run_caesar_encrypt(text: &str, shift: i16) -> String {
    let mut result: String = String::new();
    for c in text.chars() {
        let mut c_char = c;
        if c_char.is_alphabetic() {
            c_char = (c_char as u8 + shift as u8) as char;
        }
        result.push(c_char);
    }
    result
}

///
/// 文字列の暗号化（シーザー暗号）
/// (I) text: &str -> 暗号化したい文字列
/// (I) shift: i16 -> 文字のシフト数
/// (R) result: String -> 暗号化後の文字列
///
fn caesar_rotate(text: &str, shift: i16) -> String {
    // 変換結果の文字列を取得するオブジェクト
    let mut result = String::new();

    // 1文字ずつ繰り返す
    for ch in text.chars() {
        // 小文字を大文字に変換する。
        let ch = if ch.is_lowercase() {
            ch.to_ascii_uppercase()
        } else {
            ch
        };
        // 大文字の時のシフト処理
        if 'A' <= ch && ch <= 'Z' {
            let a = 'A' as i16;
            let enc = (((ch as i16) - a + shift + 26) % 26 + a) as u8;
            result.push(enc as char);
        } else {
            result.push(ch as char);
        }
    }
    return result;
}

///
/// シダを描画する
///
fn draw_fern(img: &mut RgbImage, k: i64, x: f64, y: f64) {
    // 計算用のクロージャを定義
    let w1x = |x, y| 0.836 * x + 0.044 * y;
    let x1y = |x, y| -0.044 * x + 0.836 * y + 0.169;
    let w2x = |x, y| -0.141 * x + 0.302 * y;
    let w2y = |x, y| 0.302 * x + 0.141 * y + 0.127;
    let w3x = |x, y| 0.141 * x - 0.302 * y;
    let w3y = |x, y| 0.302 * x + 0.141 * y + 0.169;
    let w4x = |_x, _y| 0.0;
    let w4y = |_x, y| 0.175337 * y;

    if k > 0 {
        // 再帰的に描画 --- (*4)
        draw_fern(img, k - 1, w1x(x, y), x1y(x, y));
        if lazyrand::rand_f64() < 0.3 {
            draw_fern(img, k - 1, w2x(x, y), w2y(x, y));
        }
        if lazyrand::rand_f64() < 0.3 {
            draw_fern(img, k - 1, w3x(x, y), w3y(x, y));
        }
        if lazyrand::rand_f64() < 0.3 {
            draw_fern(img, k - 1, w4x(x, y), w4y(x, y));
        }
    }

    // 座標を計算 --- (*5)
    let ss = HEIGHT as f64 * 0.97;
    let xx = (x * ss + (WIDTH as f64) * 0.5) as u32 - 1;
    let yy = ((HEIGHT as f64) - y * ss) as u32 - 1;
    // 描画 --- (*6)
    img.put_pixel(xx, yy, Rgb([120, 255, 110]));
}

///
/// フィボナッチ数列
///          0 | n = 0
/// F(n) = { 1 | n = 1
///          F(n-1) + F(n-2) | n >= 2
///
fn calc_fib(n_count: i32, h: &mut HashMap<i32, i32>) -> i32 {
    if n_count == 0 || n_count == 1 {
        h.entry(n_count).or_insert(n_count);
        n_count
    } else {
        if h.get(&n_count) != None {
            *h.get(&n_count).unwrap()
        } else {
            let result = calc_fib(n_count - 1, h) + calc_fib(n_count - 2, h);
            h.entry(n_count).or_insert(result);
            result
        }
    }
}
