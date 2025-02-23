use image::{Rgb, RgbImage};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const SAVE_FILE: &str = "image.png";

///
/// シダ植物の描画
///
#[allow(unused)]
pub fn run_draw_fern() {
    // シダの描画
    // 画像バッファの作成
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // 描画
    draw_fern(&mut img, 23, 0.0, 0.0);

    // ファイルに保存する
    image::save_buffer(SAVE_FILE, &img, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
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
