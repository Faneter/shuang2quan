mod converter;
mod scheme;

use converter::ShuangPinConverter;
use scheme::ShuangPinScheme;

fn print_usage(program: &str) {
    eprintln!("用法: {} [选项] <双拼文本>", program);
    eprintln!();
    eprintln!("选项:");
    eprintln!("  -s, --scheme <方案>   指定双拼方案 (默认: xiaohe)");
    eprintln!("  -h, --help            显示帮助信息");
    eprintln!("  -l, --list            列出所有支持的双拼方案");
    eprintln!();
    eprintln!("支持的双拼方案:");
    for name in ShuangPinScheme::all_names() {
        eprintln!("  - {}", name);
    }
    eprintln!();
    eprintln!("示例:");
    eprintln!("  {} -s xiaohe \"ud pb\"        # 小鹤双拼转全拼", program);
    eprintln!("  {} -s microsoft \"ud pb\"     # 微软双拼转全拼", program);
    eprintln!(
        "  {} \"ud pb\"                  # 使用默认方案转换",
        program
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0];

    if args.len() < 2 {
        print_usage(program);
        std::process::exit(1);
    }

    let mut scheme = ShuangPinScheme::XiaoHe;
    let mut input = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage(program);
                std::process::exit(0);
            }
            "-l" | "--list" => {
                println!("支持的双拼方案:");
                for name in ShuangPinScheme::all_names() {
                    println!("  - {}", name);
                }
                std::process::exit(0);
            }
            "-s" | "--scheme" => {
                if i + 1 < args.len() {
                    if let Some(s) = ShuangPinScheme::from_name(&args[i + 1]) {
                        scheme = s;
                        i += 2;
                    } else {
                        eprintln!("错误: 未知的双拼方案 '{}'", args[i + 1]);
                        eprintln!("使用 --list 查看支持的方案");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("错误: --scheme 需要参数");
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

    let converter = ShuangPinConverter::new(scheme);
    let result = converter.convert(&input);
    println!("{}", result);
}
