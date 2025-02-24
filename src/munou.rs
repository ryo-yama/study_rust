use std::io::{Result};
use std::collections::HashMap;
use std::fs;
use regex::Regex;
use vibrato::{Dictionary, Tokenizer};
use lazyrand;
use std::io::{self, Write};

// 特殊な単語 ID
static TOP_WORD_ID: isize = 0;
static END_WORD_ID: isize = 1;


///
/// マルコフ連鎖を用いている構造体
///
pub struct MarkovChain {
    words: Vec<String>, // 単語リスト
    word_hash: HashMap<String, isize>, // 単語と ID のハッシュマップ
    chain: HashMap<(isize, isize), Vec<isize>>, // マルコフ連鎖の辞書
    tokenizer: Tokenizer, // 形態素解析器
}

///
/// マルコフ連鎖の実装部分
///
impl MarkovChain {
    pub fn new() -> Self {
        // 形態素解析の辞書を読み込む
        let reader = zstd::Decoder::new(fs::File::open("asset/system.dic.zst").unwrap()).unwrap();
        let dict = Dictionary::read(reader).unwrap();

        // 単語辞書の初期化
        let mut word_hash = HashMap::new();
        word_hash.insert("*".to_string(), TOP_WORD_ID);
        word_hash.insert("。".to_string(), END_WORD_ID);
        let mut words = Vec::new();
        words.push("*".to_string());
        words.push("。".to_string());

        // 構造体の作成
        MarkovChain {
            words,
            word_hash,
            chain: HashMap::new(),
            tokenizer: Tokenizer::new(dict),
        }
    }
}

pub fn execute() -> Result<()> {
    Ok(())
}