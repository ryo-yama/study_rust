use std::collections::HashMap;

///
/// フィボナッチ数列
///
#[allow(unused)]
pub fn run_fib(input_num: i32) {
    // 整数
    let n_count: i32 = input_num;

    // 計算結果を保持する配列c
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    // フィボナッチ数列の計算
    let n_result: i32 = calc_fib(10, &mut hash_map);
    println!("calc_fibo {} is {}.", n_count, n_result);
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
