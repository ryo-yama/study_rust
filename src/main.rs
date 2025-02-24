mod fern;
mod my_math;
mod caesar_crypt;
mod life_game;
mod munou;

use std::io::Result;
use std::{env, process};
use study_rust::Config;

///
/// main 関数
///
fn main() -> Result<()> {
    // シダの描画
    // fern::run_draw_fern();
    // フィボナッチ数列
    // my_math::run_fib(10);
    // シーザー暗号
    // caesar_crypt::execute("Hello World!\n");

    // ライフゲーム
    life_game::run()?;
    // 人工無能
    munou::execute()?;
    Ok(())
}

///
/// ファイルを探すプログラム
///
#[allow(unused)]
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
