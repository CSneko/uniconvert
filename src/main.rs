use std::env;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了足够的参数
    if args.len() < 3 {
        println!("Not enough arguments provided.");
        return;
    }

    // 根据参数执行相应的操作
    match args[1].as_str() {
        "-t" | "--text" => {
            let text = &args[2];
            println!("{}", text_to_unicode(text));
        }
        "-u" | "--unicode" => {
            let unicode = &args[2];
            println!("{}", unicode_to_text(unicode));
        }
        "-h" | "--help" => {
            // 打印帮助信息
            print_help();
        }
        _ => {
            println!("Unknown option: {}", args[1]);
        }
    }

}

/// 将文本转换为 Unicode 编码的字符串表示形式。
///
/// # 参数
///
/// - `text`: 要转换的文本字符串。
///
/// # 返回值
///
/// 返回一个包含文本的 Unicode 编码的字符串。
///
/// # 示例
///
/// ```rust
/// let unicode_string = text_to_unicode("Hello");
/// println!("Unicode representation: {}", unicode_string);
/// ```
fn text_to_unicode(text: &str) -> String {
    return  text.chars().map(|c| format!("\\u{:04X}", c as u32)).collect();
}

/// 将 Unicode 编码的字符串转换为文本。
///
/// # 参数
///
/// - `unicode`: Unicode 编码的字符串，格式为 `\u{XXXX}`，其中 XXXX 是 Unicode 码点的十六进制表示。
///
/// # 返回值
///
/// 返回包含解码后文本的字符串。
///
/// # 示例
///
/// ```rust
/// let text = unicode_to_text("u0048u0065u006cu006cu006f");
/// println!("Decoded text: {}", text); // 输出 "Hello"
/// ```
fn unicode_to_text(input: &str) -> String {
    // 使用正则表达式匹配 `\u{XXXX}` 格式的子串
    let re = regex::Regex::new(r#"(?i)\\u\{([0-9a-f]{4})\}"#).unwrap();

    // 使用正则表达式的 replace_all 方法将匹配到的 `\u{XXXX}` 替换为相应的 Unicode 字符
    let replaced = re.replace_all(input, |caps: &regex::Captures| {
        let hex_code = &caps[1]; // 提取匹配到的 XXXX 部分
        let code_point = u32::from_str_radix(hex_code, 16).unwrap(); // 解析为十六进制的 Unicode 码点
        std::char::from_u32(code_point).unwrap().to_string() // 转换为 Unicode 字符并返回
    });

    // 返回解码后的文本
    replaced.into_owned()
}



/// 打印帮助信息
fn print_help() {
    println!(
        r#"
        Unicode and Text Convert

        Usage: uniconvert [OPTION] [SOURCE]

        Options:
            -h, --help         Print help information
            -t, --text         Convert text to unicode
            -u, --unicode      Convert unicode to text
        "#
    );
}
