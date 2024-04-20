//! # captcha

use std::{env, io};

use captcha_rs::CaptchaBuilder;

use captcha::{args_map, get_arg_value, print_help, validate_length, validate_text};

/// 主函数
///
/// ```shell
/// ./captcha -h
/// ```
fn main() -> io::Result<()> {

    // 获取所有的命令行参数
    let args: Vec<String> = env::args().collect();

    // 当输入 -h 或者 -help 时，输出提示：
    if args.len() == 2 && (args[1] == "-h" || args[1] == "-help") {
        print_help();
        return Ok(());
    }

    // 参数 map
    let args_map = args_map(&args);

    // 文本
    let text = get_arg_value(&args_map, "-t", "--text").unwrap_or(String::new());
    validate_text(&text)?;
    // 长度
    let length = get_arg_value(&args_map, "-l", "--length").unwrap_or(if text.len() > 0 { text.len().to_string() } else { 5.to_string() }).parse::<usize>().unwrap();
    validate_length(length)?;
    // 宽度
    let width = get_arg_value(&args_map, "-w", "--width").unwrap_or((length * 26).to_string()).parse::<u32>().unwrap();
    // 高度
    let height = get_arg_value(&args_map, "-h", "--height").unwrap_or(40.to_string()).parse::<u32>().unwrap();
    // 深色模式
    let dark_mode = get_arg_value(&args_map, "-d", "--dark-mode").unwrap_or(false.to_string()).parse::<bool>().unwrap();
    // 复杂度
    let complexity = get_arg_value(&args_map, "-c", "--complexity").unwrap_or(1.to_string()).parse::<u32>().unwrap();
    // 压缩度
    let compression = get_arg_value(&args_map, "-C", "--compression").unwrap_or(40.to_string()).parse::<u8>().unwrap();

    let builder: CaptchaBuilder = if text.len() > 0 {
        CaptchaBuilder::new()
            .text(text)
    } else {
        CaptchaBuilder::new()
            .length(length) // min: 1, max: 10, default: 5
    };

    let captcha = builder
        .width(width) // min: 80, max: 320, default: 130
        .height(height) // min: 30, max: 120, default: 40
        .dark_mode(dark_mode) // default: false
        .complexity(complexity) // min: 1, max: 10, default: 1
        .compression(compression) // min: 1, max: 99, default: 40
        .build();

    println!("{}", captcha.text);
    println!("{}", captcha.to_base64());

    return Ok(());
}
