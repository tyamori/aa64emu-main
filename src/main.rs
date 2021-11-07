use std::{env, fs};
mod eval;
mod parser;

fn main() {
    // コマンドライン引数の検査
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("以下のようにファイル名を指定して実行してください\ncargo run examples/ex1.S");
        return;
    }

    // ファイル読み込み
    let content = match fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            println!("エラー: {:?}", e);
            return;
        }
    };

    // パース
    let result = parser::parse_asm(&content);
    match result {
        Ok((_, ops)) => {
            let ctx = eval::run(&ops); // 実行
            println!("result: {:#?}", ctx); // 実行結果を表示
        }
        Err(e) => {
            println!("parse error: {:#?}", e); // エラーを表示
        }
    }
}
