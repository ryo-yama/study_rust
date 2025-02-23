///
/// シーザー暗号化の実行
///
#[allow(unused)]
pub fn execute(input_str: &str) {
    // シーザー暗号
    let tgt_str = input_str;
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
