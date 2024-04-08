use std::{env, io};
use std::collections::HashMap;

use captcha_rs::CaptchaBuilder;

fn main() -> io::Result<()> {

    // 获取所有的命令行参数
    let args: Vec<String> = env::args().collect();

    // 当输入 -h 或者 -help 时，输出提示：
    if args.len() == 2 && (args[1] == "-h" || args[1] == "-help") {
        print_help();
        return Ok(());
    }

    // 命令和参数示例：
    let args_map = args_map(args);

    // 从参数 -t 或者 --test 中获取验证码文本
    let text = match args_map.get("-t") {
        Some(value) => value[0].clone(),
        None => match args_map.get("--text") {
            Some(value) => value[0].clone(),
            None => String::from(""),
        }
    };
    if text.len() > 10 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "The length of the text must be less than 10"));
    }
    if text.contains(|c: char| EXCLUDE.contains(c)) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("The text must not contain [{}]", EXCLUDE)));
    }

    // 从参数 -l 或者 --length 中获取验证码文本长度
    let length = if text.len() > 0 {
        text.len()
    } else {
        match args_map.get("-l") {
            Some(value) => value[0].parse::<usize>().unwrap(),
            None => match args_map.get("--length") {
                Some(value) => value[0].parse::<usize>().unwrap(),
                None => 5,
            }
        }
    };
    if length < 1 || length > 10 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "The length of the text must be between 1 and 10"));
    }

    // 从参数 -w 或者 --width 中获取验证码图像宽度
    let width = match args_map.get("-w") {
        Some(value) => value[0].parse::<u32>().unwrap(),
        None => match args_map.get("--width") {
            Some(value) => value[0].parse::<u32>().unwrap(),
            None => length as u32 * 26,
        }
    };

    // 从参数 -h 或者 --height 中获取验证码图像高度
    let height = match args_map.get("-h") {
        Some(value) => value[0].parse::<u32>().unwrap(),
        None => match args_map.get("--height") {
            Some(value) => value[0].parse::<u32>().unwrap(),
            None => 40,
        }
    };

    // 从参数 -d 或者 --dark-mode 中获取验证码图像是否为暗色模式
    let dark_mode = match args_map.get("-d") {
        Some(value) => value[0].parse::<bool>().unwrap(),
        None => match args_map.get("--dark-mode") {
            Some(value) => value[0].parse::<bool>().unwrap(),
            None => false,
        }
    };

    // 从参数 -c 或者 --complexity 中获取验证码复杂度
    let complexity = match args_map.get("-c") {
        Some(value) => value[0].parse::<u32>().unwrap(),
        None => match args_map.get("--complexity") {
            Some(value) => value[0].parse::<u32>().unwrap(),
            None => 1,
        }
    };

    // 从参数 -C 或者 --compression 中获取验证码压缩率
    let compression = match args_map.get("-C") {
        Some(value) => value[0].parse::<u8>().unwrap(),
        None => match args_map.get("--compression") {
            Some(value) => value[0].parse::<u8>().unwrap(),
            None => 40,
        }
    };

    let builder: CaptchaBuilder = if text.len() > 0 {
        CaptchaBuilder::new().text(text)
    } else {
        CaptchaBuilder::new()
    };

    let captcha = builder
        .length(length) // min: 1, max: 10, default: 5
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

const EXCLUDE: &str = "01ILOilo";

/**
 * 获取所有的命令行参数，然后创建一个 HashMap 来存储参数名和参数值。然后，我们遍历所有的参数，
 * 如果一个参数以 - 开头，把它作为参数名，把它后面的参数（如果存在且不以 - 开头）作为参数值
 */
fn args_map(args: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let mut key = String::new();
    for arg in args.iter() {
        if arg.starts_with("-") {
            key = arg.clone();
            map.insert(key.clone(), Vec::new());
        } else {
            match map.get_mut(&key) {
                Some(value) => value.push(arg.clone()),
                None => println!("Key does not exist in HashMap"),
            }
        }
    }
    return map;
}

fn print_help() {
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
