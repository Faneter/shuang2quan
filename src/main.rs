use std::collections::HashMap;

/// 双拼方案枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShuangPinScheme {
    XiaoHe,     // 小鹤双拼
    Microsoft,  // 微软双拼
    SouGou,     // 搜狗双拼
    ZiRanMa,    // 自然码双拼
    ZhiNengABC, // 智能ABC双拼
    ZiGuang,    // 紫光双拼
}

impl ShuangPinScheme {
    pub fn all_names() -> &'static [&'static str] {
        &[
            "xiaohe",
            "microsoft",
            "sougou",
            "ziranma",
            "zhinengabc",
            "ziguang",
        ]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "xiaohe" => Some(ShuangPinScheme::XiaoHe),
            "microsoft" => Some(ShuangPinScheme::Microsoft),
            "sougou" | "sogou" => Some(ShuangPinScheme::SouGou),
            "ziranma" => Some(ShuangPinScheme::ZiRanMa),
            "zhinengabc" => Some(ShuangPinScheme::ZhiNengABC),
            "ziguang" => Some(ShuangPinScheme::ZiGuang),
            _ => None,
        }
    }
}

/// 双拼方案数据：包含声母映射和韵母映射
/// 对于共享键位的韵母（如 iang/uang），使用需要根据声母区分的映射
struct SchemeData {
    initials: HashMap<char, &'static str>,
    finals: HashMap<char, &'static str>,
    /// 条件韵母映射: (声母模式, 键位) -> 韵母
    /// 用于处理 iang/uang, ong/iong 等共享键位的情况
    conditional_finals: Vec<(&'static str, char, &'static str)>,
}

/// 获取指定方案的双拼数据
fn get_scheme_data(scheme: ShuangPinScheme) -> SchemeData {
    let mut initials = HashMap::new();
    let mut finals = HashMap::new();
    let mut conditional_finals = Vec::new();

    // 基本声母 (所有方案通用)
    for &(c, s) in &[
        ('b', "b"),
        ('c', "c"),
        ('d', "d"),
        ('f', "f"),
        ('g', "g"),
        ('h', "h"),
        ('j', "j"),
        ('k', "k"),
        ('l', "l"),
        ('m', "m"),
        ('n', "n"),
        ('p', "p"),
        ('q', "q"),
        ('r', "r"),
        ('s', "s"),
        ('t', "t"),
        ('w', "w"),
        ('x', "x"),
        ('y', "y"),
        ('z', "z"),
    ] {
        initials.insert(c, s);
    }

    match scheme {
        ShuangPinScheme::XiaoHe => {
            // 小鹤双拼声母: zh->v, ch->i, sh->u
            initials.insert('v', "zh");
            initials.insert('i', "ch");
            initials.insert('u', "sh");

            // 小鹤双拼完整韵母映射
            for &(c, s) in &[
                ('a', "a"),
                ('o', "o"),
                ('e', "e"),
                ('i', "i"),
                ('u', "u"),
                ('v', "ui"),
                ('b', "in"),
                ('c', "ao"),
                ('f', "en"),
                ('g', "eng"),
                ('h', "ang"),
                ('j', "an"),
                ('k', "ing"),
                ('l', "ai"),
                ('m', "ian"),
                ('n', "iao"),
                ('p', "ie"),
                ('q', "iu"),
                ('r', "uan"),
                ('t', "ue"),
                ('w', "ei"),
                ('x', "ia"),
                ('y', "un"),
                ('z', "ou"),
            ] {
                finals.insert(c, s);
            }

            // 条件韵母映射
            // d: iang(零声母/j/q/x/y) / uang(其他)
            conditional_finals.push(("jqx", 'd', "iang"));
            conditional_finals.push(("y", 'd', "iang"));
            conditional_finals.push(("", 'd', "uang"));
            // s: ong(零声母/s/sh) / iong(其他)
            conditional_finals.push(("jqx", 's', "iong"));
            conditional_finals.push(("y", 's', "iong"));
            conditional_finals.push(("", 's', "ong"));
        }
        ShuangPinScheme::Microsoft | ShuangPinScheme::SouGou => {
            // 微软/搜狗双拼声母: zh->a, ch->o, sh->e
            initials.insert('a', "zh");
            initials.insert('o', "ch");
            initials.insert('e', "sh");

            // 微软双拼完整韵母映射
            for &(c, s) in &[
                ('a', "a"),
                ('o', "o"),
                ('e', "e"),
                ('i', "i"),
                ('u', "u"),
                ('v', "ui"),
                ('b', "in"),
                ('c', "ao"),
                ('f', "en"),
                ('g', "eng"),
                ('h', "ang"),
                ('j', "an"),
                ('k', "ing"),
                ('l', "ai"),
                ('m', "ian"),
                ('n', "iao"),
                ('p', "ie"),
                ('q', "iu"),
                ('r', "uan"),
                ('t', "ue"),
                ('w', "ei"),
                ('x', "ia"),
                ('y', "un"),
                ('z', "ou"),
                (';', "ing"),
            ] {
                finals.insert(c, s);
            }

            // 条件韵母映射
            conditional_finals.push(("jqx", 'd', "iang"));
            conditional_finals.push(("y", 'd', "iang"));
            conditional_finals.push(("", 'd', "uang"));
            conditional_finals.push(("jqx", 's', "iong"));
            conditional_finals.push(("y", 's', "iong"));
            conditional_finals.push(("", 's', "ong"));
        }
        ShuangPinScheme::ZiRanMa => {
            // 自然码声母: zh->v, ch->i, sh->u
            initials.insert('v', "zh");
            initials.insert('i', "ch");
            initials.insert('u', "sh");

            // 自然码完整韵母映射
            for &(c, s) in &[
                ('a', "a"),
                ('o', "o"),
                ('e', "e"),
                ('i', "i"),
                ('u', "u"),
                ('v', "ui"),
                ('b', "in"),
                ('c', "ao"),
                ('f', "en"),
                ('g', "eng"),
                ('h', "ang"),
                ('j', "an"),
                ('k', "ing"),
                ('l', "ai"),
                ('m', "ian"),
                ('n', "iao"),
                ('p', "ie"),
                ('q', "iu"),
                ('r', "uan"),
                ('t', "ue"),
                ('w', "ei"),
                ('x', "ia"),
                ('y', "un"),
                ('z', "ou"),
                (';', "ing"),
            ] {
                finals.insert(c, s);
            }

            // 条件韵母映射
            conditional_finals.push(("jqx", 'd', "iang"));
            conditional_finals.push(("y", 'd', "iang"));
            conditional_finals.push(("", 'd', "uang"));
            conditional_finals.push(("jqx", 's', "iong"));
            conditional_finals.push(("y", 's', "iong"));
            conditional_finals.push(("", 's', "ong"));
        }
        ShuangPinScheme::ZhiNengABC => {
            // 智能ABC声母: zh->a, ch->e, sh->v
            initials.insert('a', "zh");
            initials.insert('e', "ch");
            initials.insert('v', "sh");

            // 智能ABC完整韵母映射
            for &(c, s) in &[
                ('a', "a"),
                ('o', "o"),
                ('e', "e"),
                ('i', "i"),
                ('u', "u"),
                ('v', "ui"),
                ('b', "in"),
                ('c', "ao"),
                ('f', "en"),
                ('g', "eng"),
                ('h', "ang"),
                ('j', "an"),
                ('k', "ing"),
                ('l', "ai"),
                ('m', "ian"),
                ('n', "iao"),
                ('p', "ie"),
                ('q', "iu"),
                ('r', "uan"),
                ('t', "ue"),
                ('w', "ei"),
                ('x', "ia"),
                ('y', "un"),
                ('z', "ou"),
                ('\'', "ing"),
            ] {
                finals.insert(c, s);
            }

            // 条件韵母映射
            conditional_finals.push(("jqx", 'd', "iang"));
            conditional_finals.push(("y", 'd', "iang"));
            conditional_finals.push(("", 'd', "uang"));
            conditional_finals.push(("jqx", 's', "iong"));
            conditional_finals.push(("y", 's', "iong"));
            conditional_finals.push(("", 's', "ong"));
        }
        ShuangPinScheme::ZiGuang => {
            // 紫光双拼声母: sh->u, ch->a, zh->i
            initials.insert('u', "sh");
            initials.insert('a', "ch");
            initials.insert('i', "zh");

            // 紫光完整韵母映射
            for &(c, s) in &[
                ('a', "a"),
                ('o', "o"),
                ('e', "e"),
                ('i', "i"),
                ('u', "u"),
                ('v', "ui"),
                ('b', "in"),
                ('c', "ao"),
                ('f', "en"),
                ('g', "eng"),
                ('h', "ang"),
                ('j', "an"),
                ('k', "ing"),
                ('l', "ai"),
                ('m', "ian"),
                ('n', "iao"),
                ('p', "ie"),
                ('q', "iu"),
                ('r', "uan"),
                ('t', "ue"),
                ('w', "ei"),
                ('x', "ia"),
                ('y', "un"),
                ('z', "ou"),
                (';', "ing"),
            ] {
                finals.insert(c, s);
            }

            // 条件韵母映射
            conditional_finals.push(("jqx", 'd', "iang"));
            conditional_finals.push(("y", 'd', "iang"));
            conditional_finals.push(("", 'd', "uang"));
            conditional_finals.push(("jqx", 's', "iong"));
            conditional_finals.push(("y", 's', "iong"));
            conditional_finals.push(("", 's', "ong"));
        }
    }

    SchemeData {
        initials,
        finals,
        conditional_finals,
    }
}

/// 双拼转换器
pub struct ShuangPinConverter {
    scheme: ShuangPinScheme,
    data: SchemeData,
}

impl ShuangPinConverter {
    /// 创建新的转换器
    pub fn new(scheme: ShuangPinScheme) -> Self {
        let data = get_scheme_data(scheme);
        ShuangPinConverter { scheme, data }
    }

    /// 获取当前方案
    pub fn scheme(&self) -> ShuangPinScheme {
        self.scheme
    }

    /// 根据声母和键位获取韵母（处理条件韵母）
    fn get_final(&self, initial: &str, key: char) -> Option<&'static str> {
        // 先检查条件韵母映射（非默认的优先）
        for &(prefixes, k, fin) in &self.data.conditional_finals {
            if k == key && !prefixes.is_empty() {
                if prefixes.chars().any(|p| initial.starts_with(p)) {
                    return Some(fin);
                }
            }
        }

        // 再检查条件韵母的默认映射
        for &(prefixes, k, fin) in &self.data.conditional_finals {
            if k == key && prefixes.is_empty() {
                return Some(fin);
            }
        }

        // 最后检查普通韵母映射
        self.data.finals.get(&key).copied()
    }

    /// 转换单个双拼编码为全拼
    pub fn convert_syllable(&self, code: &str) -> Option<String> {
        if code.is_empty() {
            return None;
        }

        let chars: Vec<char> = code.chars().collect();

        // 处理零声母音节（单字符）
        if chars.len() == 1 {
            let c = chars[0];
            // 如果是韵母映射中的字符，直接返回对应的韵母
            if let Some(final_str) = self.data.finals.get(&c) {
                return Some(final_str.to_string());
            }
            // 如果是 a, o, e 等单韵母，直接返回
            if "aoe".contains(c) {
                return Some(c.to_string());
            }
            return None;
        }

        // 处理普通双拼音节（两个字符）
        if chars.len() == 2 {
            let initial_char = chars[0];
            let final_char = chars[1];

            // 获取声母
            let initial = if let Some(s) = self.data.initials.get(&initial_char) {
                s.to_string()
            } else {
                // 如果不是特殊声母，尝试直接使用该字符作为声母
                initial_char.to_string()
            };

            // 获取韵母（处理条件韵母）
            let final_str = if let Some(s) = self.get_final(&initial, final_char) {
                s.to_string()
            } else if let Some(s) = self.data.finals.get(&final_char) {
                s.to_string()
            } else {
                final_char.to_string()
            };

            return Some(initial + &final_str);
        }

        None
    }

    /// 转换一段双拼文本为全拼
    pub fn convert(&self, input: &str) -> String {
        let input = input.to_lowercase();
        let chars: Vec<char> = input.chars().collect();
        let mut result = String::new();
        let mut i = 0;

        while i < chars.len() {
            // 跳过空格
            if chars[i].is_whitespace() {
                if !result.is_empty() && !result.ends_with(' ') {
                    result.push(' ');
                }
                i += 1;
                continue;
            }

            // 尝试取两个字符作为一个音节
            if i + 1 < chars.len() && !chars[i + 1].is_whitespace() {
                let syllable: String = chars[i..=i + 1].iter().collect();
                if let Some(quanpin) = self.convert_syllable(&syllable) {
                    if !result.is_empty() && !result.ends_with(' ') {
                        result.push('\'');
                    }
                    result.push_str(&quanpin);
                    i += 2;
                    continue;
                }
            }

            // 如果两个字符无法解析，尝试单个字符（零声母）
            let single = chars[i].to_string();
            if let Some(quanpin) = self.convert_syllable(&single) {
                if !result.is_empty() && !result.ends_with(' ') {
                    result.push('\'');
                }
                result.push_str(&quanpin);
            } else {
                // 无法解析，保留原字符
                result.push(chars[i]);
            }
            i += 1;
        }

        result
    }

    /// 转换并保留原始分隔（空格分隔的音节）
    pub fn convert_separated(&self, input: &str) -> String {
        let input = input.to_lowercase();
        let syllables: Vec<&str> = input.split_whitespace().collect();
        let mut result = Vec::new();

        for syllable in syllables {
            if let Some(quanpin) = self.convert_syllable(syllable) {
                result.push(quanpin);
            } else {
                result.push(syllable.to_string());
            }
        }

        result.join(" ")
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xiaohe_shuang() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        // 小鹤双拼: sh->u, uang->d (sh 声母匹配 uang)
        assert_eq!(converter.convert_syllable("ud"), Some("shuang".to_string()));
    }

    #[test]
    fn test_xiaohe_jiang() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        // 小鹤双拼: j->j, iang->d (j 声母匹配 iang)
        assert_eq!(converter.convert_syllable("jd"), Some("jiang".to_string()));
    }

    #[test]
    fn test_xiaohe_qiong() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        // 小鹤双拼: q->q, iong->s (q 声母匹配 iong)
        assert_eq!(converter.convert_syllable("qs"), Some("qiong".to_string()));
    }

    #[test]
    fn test_xiaohe_song() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        // 小鹤双拼: s->s, ong->s (s 声母匹配 ong)
        assert_eq!(converter.convert_syllable("ss"), Some("song".to_string()));
    }

    #[test]
    fn test_xiaohe_pin() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        // 小鹤双拼: p->p, in->b, 所以 "pin" = "pb"
        assert_eq!(converter.convert_syllable("pb"), Some("pin".to_string()));
    }

    #[test]
    fn test_xiaohe_full() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        // "ud pb" -> "shuang'pin"
        assert_eq!(converter.convert("ud pb"), "shuang pin");
    }

    #[test]
    fn test_microsoft_shuang() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::Microsoft);
        // 微软双拼: sh->e, uang->d, 所以 "shuang" = "ed"
        assert_eq!(converter.convert_syllable("ed"), Some("shuang".to_string()));
    }

    #[test]
    fn test_microsoft_jiang() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::Microsoft);
        // 微软双拼: j->j, iang->d
        assert_eq!(converter.convert_syllable("jd"), Some("jiang".to_string()));
    }

    #[test]
    fn test_microsoft_pin() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::Microsoft);
        // 微软双拼: p->p, in->b, 所以 "pin" = "pb"
        assert_eq!(converter.convert_syllable("pb"), Some("pin".to_string()));
    }

    #[test]
    fn test_scheme_from_name() {
        assert_eq!(
            ShuangPinScheme::from_name("xiaohe"),
            Some(ShuangPinScheme::XiaoHe)
        );
        assert_eq!(
            ShuangPinScheme::from_name("microsoft"),
            Some(ShuangPinScheme::Microsoft)
        );
        assert_eq!(
            ShuangPinScheme::from_name("sougou"),
            Some(ShuangPinScheme::SouGou)
        );
        assert_eq!(ShuangPinScheme::from_name("unknown"), None);
    }

    #[test]
    fn test_empty_input() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        assert_eq!(converter.convert(""), "");
    }

    #[test]
    fn test_single_char() {
        let converter = ShuangPinConverter::new(ShuangPinScheme::XiaoHe);
        assert_eq!(converter.convert_syllable("a"), Some("a".to_string()));
    }
}
