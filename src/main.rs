mod converter;
mod scheme;

use converter::ShuangPinConverter;
use scheme::SchemeData;
use std::fs;

fn print_usage(program: &str) {
    eprintln!("用法: {} [选项] <双拼文本>", program);
    eprintln!();
    eprintln!("选项:");
    eprintln!("  -s, --scheme <方案>   指定内置双拼方案 (默认: xiaohe)");
    eprintln!("  -c, --config <文件>   从配置文件加载自定义方案");
    eprintln!("  -h, --help            显示帮助信息");
    eprintln!("  -l, --list            列出所有支持的双拼方案");
    eprintln!();
    eprintln!("内置双拼方案:");
    for name in SchemeData::builtin_names() {
        eprintln!("  - {}", name);
    }
    eprintln!();
    eprintln!("配置文件格式:");
    eprintln!("  initial:v=zh         # 声母映射: 键=全拼声母");
    eprintln!("  final:b=in           # 韵母映射: 键=全拼韵母");
    eprintln!("  conditional:jqx,d=iang  # 条件韵母: 声母前缀,键=韵母");
    eprintln!("  conditional:,d=uang     # 默认条件: 空前缀表示默认匹配");
    eprintln!();
    eprintln!("示例:");
    eprintln!(
        "  {} -s xiaohe \"ud pb\"        # 使用内置小鹤双拼",
        program
    );
    eprintln!("  {} -c myscheme.txt \"ud pb\"  # 使用自定义方案", program);
    eprintln!("  {} \"ud pb\"                  # 使用默认方案", program);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        print_usage(program);
        std::process::exit(1);
    }

    let mut scheme_name = "xiaohe".to_string();
    let mut config_path: Option<String> = None;
    let mut input = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage(program);
                std::process::exit(0);
            }
            "-l" | "--list" => {
                println!("内置双拼方案:");
                for name in SchemeData::builtin_names() {
                    println!("  - {}", name);
                }
                std::process::exit(0);
            }
            "-s" | "--scheme" => {
                if i + 1 < args.len() {
                    scheme_name = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("错误: --scheme 需要参数");
                    std::process::exit(1);
                }
            }
            "-c" | "--config" => {
                if i + 1 < args.len() {
                    config_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("错误: --config 需要参数");
                    std::process::exit(1);
                }
            }
            _ => {
                if input.is_empty() {
                    input = args[i].clone();
                } else {
                    input.push(' ');
                    input.push_str(&args[i]);
                }
                i += 1;
            }
        }
    }

    if input.is_empty() {
        use std::io::{self, Read};
        let mut buffer = String::new();
        if io::stdin().read_to_string(&mut buffer).is_ok() {
            input = buffer.trim().to_string();
        }
    }

    if input.is_empty() {
        eprintln!("错误: 没有输入文本");
        print_usage(program);
        std::process::exit(1);
    }

    // 创建转换器
    let converter = if let Some(path) = config_path {
        // 从配置文件加载
        match fs::read_to_string(&path) {
            Ok(content) => match SchemeData::from_str(&content) {
                Ok(data) => ShuangPinConverter::new(data),
                Err(e) => {
                    eprintln!("错误: 解析配置文件失败: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("错误: 读取配置文件失败 '{}': {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        // 使用内置方案
        match ShuangPinConverter::from_name(&scheme_name) {
            Some(c) => c,
            None => {
                eprintln!("错误: 未知的双拼方案 '{}'", scheme_name);
                eprintln!("使用 --list 查看支持的方案");
                std::process::exit(1);
            }
        }
    };

    let result = converter.convert(&input);
    println!("{}", result);
}
