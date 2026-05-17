use std::collections::HashMap;

/// 双拼方案枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShuangPinScheme {
    /// 小鹤双拼
    XiaoHe,
    /// 微软双拼
    Microsoft,
    /// 搜狗双拼
    SouGou,
    /// 自然码双拼
    ZiRanMa,
    /// 智能ABC双拼
    ZhiNengABC,
    /// 紫光双拼
    ZiGuang,
}

impl ShuangPinScheme {
    /// 获取所有支持的方案名称
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

    /// 从名称解析方案
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
pub struct SchemeData {
    pub initials: HashMap<char, &'static str>,
    pub finals: HashMap<char, &'static str>,
    /// 条件韵母映射: (声母前缀, 键位, 韵母)
    /// 用于处理 iang/uang、ong/iong 等共享键位的情况
    pub conditional_finals: Vec<(&'static str, char, &'static str)>,
}

/// 获取指定方案的双拼数据
pub fn get_scheme_data(scheme: ShuangPinScheme) -> SchemeData {
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
