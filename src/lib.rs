//! # captcha
//!
//! `captcha` 是一个命令行程序，使用 rust 语言开发，用于生成图像验证码。它将返回一个随机的验证码文本和图像的 base64 格式字符串。
//!
//! `captcha` is a command-line program developed using the rust language to generate image verification codes. It will return a random verification code text and a base64 format string of the image.
//!
//! ## 使用方法 Usage
//!
//! ```shell
//! ./captcha [options...]
//! ```
//!
//! ## 选项 Options
//!
//! - `-h, --help` 获取命令和选项的帮助
//! - `-t, --text` 设置验证码文本，由数字和大小写字母组成，排除 [01ILOilo]，长度范围为 1 到 10。默认值为随机生成 length 个字符。
//! - `-l, --length` 设置验证码文本长度，范围为 1 到 10，默认值为 5。如果设置了 -t 或 --text，此选项将被忽略。
//! - `-w, --width` 设置验证码图像宽度，范围为 80 到 320，默认值为 130
//! - `-h, --height` 设置验证码图像高度，范围为 30 到 120，默认值为 40
//! - `-d, --dark-mode` 设置验证码图像为暗色模式，默认值为 false
//! - `-c, --complexity` 设置验证码复杂度，范围为 1 到 10，值越大，验证码越复杂，默认值为 1
//! - `-C, --compression` 设置验证码压缩率，范围为 1 到 99，值越大，压缩率越高，默认值为 40
//!
//! ## 示例 Example
//!
//! ```shell
//! ./captcha -l 5 -w 130 -h 40 -d false -c 1 -C 40
//! ```

use std::collections::HashMap;
use std::io;

/// 需要从验证码文本中排除的字符。
///
/// The characters that need to be excluded from the verification code text.
pub const EXCLUDE: &str = "01ILOilo";

/// 打印命令行程序的帮助信息。
///
/// Prints help information for the command-line program.
pub fn print_help() {
    println!("这是一个使用 rust 语言开发的命令行程序，用于生成图片验证码。它将返回一个随机的验证码文本和图像的 base64 格式字符串。");
    println!("This is a command-line program developed using the rust language to generate image verification codes. It will return a random verification code text and a base64 format string of the image.");
    println!("Usage:");
    println!("  ./captcha [options...]");
    println!("Options:");
    println!("  -h, --help        获取命令和选项的帮助");
    println!("                    Get help for command and options");
    println!("  -t, --text        设置验证码文本，由数字和大小写字母组成，排除 [{}]，长度范围为 1 到 10。默认值为随机生成 length 个字符。", EXCLUDE);
    println!("                    Set captcha text, composed of numbers and uppercase and lowercase letters, excluding [{}], length range from 1 to 10. Default value is randomly generated length characters.", EXCLUDE);
    println!("  -l, --length      设置验证码文本长度，范围为 1 到 10，默认值为 5。如果设置了 -t 或 --text，此选项将被忽略。");
    println!("                    Set captcha text length, range from 1 to 10, default value is 4. If -t or --text is set, this option will be ignored.");
    println!("  -w, --width       设置验证码图像宽度，范围为 80 到 320，默认值为 130");
    println!("                    Set captcha image width, range from 80 to 320, default value is 130");
    println!("  -h, --height      设置验证码图像高度，范围为 30 到 120，默认值为 40");
    println!("                    Set captcha image height, range from 30 to 120, default value is 40");
    println!("  -d, --dark-mode   设置验证码图像为暗色模式，默认值为 false");
    println!("                    Set captcha image to dark mode, default value is false");
    println!("  -c, --complexity  设置验证码复杂度，范围为 1 到 10，值越大，验证码越复杂，默认值为 1");
    println!("                    Set captcha complexity, range from 1 to 10, the larger the value, the more complex the captcha, default value is 1");
    println!("  -C, --compression 设置验证码压缩率，范围为 1 到 99，值越大，压缩率越高，默认值为 40");
    println!("                    Set captcha compression, range from 1 to 99, the larger the value, the higher the compression rate, default value is 40");
    println!("Example:");
    println!("  ./captcha -l 5 -w 130 -h 40 -d false -c 1 -C 40");
}


/// 获取所有的命令行参数，然后创建一个 HashMap 来存储参数名和参数值。
///
/// Get all command-line arguments, then create a HashMap to store the argument name and argument value.
pub fn args_map(args: &[String]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let mut key = String::new();
    // 遍历所有的参数，如果一个参数以 - 开头，把它作为参数名，把它后面的参数（如果存在且不以 - 开头）作为参数值。
    for arg in args.iter() {
        if arg.starts_with("-") {
            key = arg.clone();
            map.insert(key.clone(), Vec::new());
        } else if let Some(value) = map.get_mut(&key) {
            value.push(arg.clone());
        }
    }
    map
}

/// 获取参数值
///
/// Get argument value
pub fn get_arg_value(args_map: &HashMap<String, Vec<String>>, short: &str, long: &str) -> Option<String> {
    args_map.get(short).and_then(|v| v.first().cloned()).or_else(|| {
        args_map.get(long).and_then(|v| v.first().cloned())
    })
}

/// 验证文本
///
/// Validate text
pub fn validate_text(text: &String) -> io::Result<()> {
    if text.len() > 10 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The length of the text must be less than 10",
        ));
    }
    if text.chars().any(|c| EXCLUDE.contains(c)) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("The text must not contain [{}]", EXCLUDE),
        ));
    }
    Ok(())
}

/// 验证文本长度
///
/// Validate text length
pub fn validate_length(length: usize) -> io::Result<()> {
    if length < 1 || length > 10 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The length of the text must be between 1 and 10",
        ));
    }
    Ok(())
}
