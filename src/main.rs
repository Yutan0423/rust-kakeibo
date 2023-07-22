use rust_kakeibo::{constants, services};
use std::io;

fn main() {
    let mut service_type = String::new();
    println!("実行したい内容を入力してください（0: 登録, 1: 集計）");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("数値を入力してください");
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        services::register::run(constants::FILE_PATH);
    } else if service_type == 1 {
        services::summarize::run(constants::FILE_PATH);
    }

    print!("実行終了");
}
