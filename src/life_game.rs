use lazyrand::rand_usize;
use std::thread;
use std::io::{Result, stdout};
use std::time::Duration;
use crossterm::{cursor, execute, terminal};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::Clear;
use crate::caesar_crypt::execute;

// 定数の宣言
const GRID_WIDTH: usize = 100; // グリッドの横幅
const GRID_HEIGHT: usize = 30; // グリッドの縦幅
const MAX_TERN: usize = 1000; // 世代数

pub fn run() -> Result<()> {
    // 画面の初期化
    init_screen()?;

    // 最初のセルを適当に配置する
    let mut cells: Vec<Vec<bool>> = init_cells();

    // 世代数だけまわす
    for i in 1..MAX_TERN + 1 {
        // セルを描画
        draw_cells(&cells)?;
        println!("{}/{}", i, MAX_TERN);

        // 300 ミリ秒待つ
        thread::sleep(Duration::from_millis(100));

        // 次の世代の計算を行う
        cells = calc_next_gen(&cells);
    }
    Ok(())
}

fn init_screen() -> Result<()> {
    // 画面をクリアして，カーソルを(0, 0)に移動
    execute!(stdout()
        , Clear(terminal::ClearType::All)
        , cursor::MoveTo(0, 0))?;
    Ok(())
}

fn init_cells() -> Vec<Vec<bool>> {
    let mut cells: Vec<Vec<bool>> = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
    // これと同義
    //let mut vec_sample = Vec::from([Vec::from([false; GRID_WIDTH]); GRID_HEIGHT]);

    // 乱数を使って適当に初期化する
    for _ in 0..(GRID_WIDTH * GRID_HEIGHT / 13) {
        cells[rand_usize() % GRID_HEIGHT][rand_usize() % GRID_WIDTH] = true;
    }
    cells
}

fn draw_cells(cells: &Vec<Vec<bool>>) -> Result<()> {
    // カーソルを(0, 0) に移動
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    for row in cells {
        for &cell in row {
            if cell {
                execute!(stdout()
                    , SetForegroundColor(Color::Yellow)
                    , SetBackgroundColor(Color::Red)
                    , Print("+"))?;
            } else {
                execute!(stdout()
                    , SetForegroundColor(Color::Blue)
                    , SetBackgroundColor(Color::Black)
                    , Print("-"))?;
            }
        }
        execute!(stdout(), Print("\n"))?;
    }
    // execute!(stdout(), ResetColor)?;
    Ok(())
}

fn calc_next_gen(cells: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // 次世代の結果を生成
    let mut new_gen_cells = vec![vec![false; GRID_WIDTH]; GRID_HEIGHT];
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            // 周囲のセルの生存数を数える
            let mut alive_cnt: u32 = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }
                    let ny = (y as isize + dy + GRID_HEIGHT as isize) as usize % GRID_HEIGHT;
                    let nx = (x as isize + dx + GRID_WIDTH as isize) as usize % GRID_WIDTH;
                    // 生存セルならインクリメント
                    if cells[ny][nx] { alive_cnt += 1; };
                }
            }
            // 以下のルールに則って，セルの生死を更新する．
            // 1. 死んでいるセルの周囲に，生きているセルが 3 つ以上いれば，そのセルに新しく誕生する．
            // 2. 生きているセルの周囲に，生きているセルが
            //    -- 1 つ以下なら，そのセルは死ぬ（過疎）
            //    -- 2 or 3 つなら，そのセルは生き続ける
            //    -- 4 以上なら，そのセルは死ぬ（過密）
            new_gen_cells[y][x] = match (cells[y][x], alive_cnt) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false
            }
        }
    }

    // 決まった次世代セルを返す
    new_gen_cells
}
