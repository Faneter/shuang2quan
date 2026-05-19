use crate::scheme::SchemeData;

/// 双拼转换器
pub struct ShuangPinConverter {
    data: SchemeData,
}

impl ShuangPinConverter {
    /// 创建新的转换器，使用自定义方案数据
    pub fn new(data: SchemeData) -> Self {
        ShuangPinConverter { data }
    }

    /// 根据名称创建转换器（使用内置方案）
    pub fn from_name(name: &str) -> Option<Self> {
        SchemeData::from_name(name).map(Self::new)
    }

    /// 根据声母和键位获取韵母（处理条件韵母）
    fn get_final(&self, initial: &str, key: char) -> Option<String> {
        let mut has_conditional = false;
        let mut default_conditional: Option<String> = None;

        // 遍历条件韵母映射
        for (prefixes, k, fin) in &self.data.conditional_finals {
            if *k == key {
                has_conditional = true;
                if prefixes.is_empty() {
                    // 记录默认映射
                    default_conditional = Some(fin.clone());
                } else if prefixes.chars().any(|p| initial.starts_with(p)) {
                    // 匹配到特定前缀
                    return Some(fin.clone());
                }
            }
        }

        // 如果有条件韵母配置，优先返回默认条件映射
        if has_conditional {
            return default_conditional;
        }

        // 最后检查普通韵母映射
        self.data.finals.get(&key).cloned()
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
                return Some(final_str.clone());
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
                s.clone()
            } else {
                // 如果不是特殊声母，尝试直接使用该字符作为声母
                initial_char.to_string()
            };

            // 获取韵母（处理条件韵母）
            let final_str = self
                .get_final(&initial, final_char)
                .or_else(|| self.data.finals.get(&final_char).cloned())
                .unwrap_or_else(|| final_char.to_string());

            return Some(initial + &final_str);
        }

        None
    }

    /// 尝试将一段文本完整解析为双拼
    ///
    /// 如果文本能被完全解析为双拼音节（每两个字符一组），返回全拼结果；
    /// 否则返回 None。
    fn try_convert_segment(&self, segment: &str) -> Option<String> {
        if segment.is_empty() {
            return Some(String::new());
        }

        let chars: Vec<char> = segment.chars().collect();
        let mut result = String::new();
        let mut i = 0;

        while i < chars.len() {
            // 尝试取两个字符作为一个音节
            if i + 1 < chars.len() {
                let syllable: String = chars[i..=i + 1].iter().collect();
                if let Some(quanpin) = self.convert_syllable(&syllable) {
                    if !result.is_empty() {
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
                if !result.is_empty() {
                    result.push('\'');
                }
                result.push_str(&quanpin);
                i += 1;
                continue;
            }

            // 无法解析，整个片段不是有效的双拼编码
            return None;
        }

        Some(result)
    }

    /// 处理一个片段，按标点符号分割后分别转换
    fn convert_fragment(&self, fragment: &str) -> String {
        if fragment.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut current_word = String::new();

        for c in fragment.chars() {
            if c.is_ascii_punctuation() && c != '\'' && c != ';' {
                // 遇到标点（保留双拼可能用到的引号和分号），先转换当前累积的单词
                if !current_word.is_empty() {
                    if let Some(converted) = self.try_convert_segment(&current_word) {
                        result.push_str(&converted);
                    } else {
                        result.push_str(&current_word);
                    }
                    current_word.clear();
                }
                // 保留标点
                result.push(c);
            } else {
                current_word.push(c);
            }
        }

        // 处理最后剩余的单词
        if !current_word.is_empty() {
            if let Some(converted) = self.try_convert_segment(&current_word) {
                result.push_str(&converted);
            } else {
                result.push_str(&current_word);
            }
        }

        result
    }

    /// 转换一段双拼文本为全拼
    ///
    /// 支持用 `` ` `` 反引号包裹文本以跳过转换：
    /// - `` `rust` `` → `rust`（保留原样）
    /// - `` `hello world` `` → `hello world`（支持空格）
    ///
    /// 未被反引号包裹的部分按正常逻辑转换。
    pub fn convert(&self, input: &str) -> String {
        let mut result = String::new();
        let mut i = 0;
        let chars: Vec<char> = input.chars().collect();

        while i < chars.len() {
            // 查找反引号起始
            if chars[i] == '`' {
                // 查找配对的结束反引号
                if let Some(end) = chars[i + 1..].iter().position(|&c| c == '`') {
                    let end = i + 1 + end;
                    // 提取反引号之间的内容（去掉反引号本身）
                    let raw: String = chars[i + 1..end].iter().collect();
                    if !result.is_empty() && !result.ends_with(' ') {
                        result.push(' ');
                    }
                    result.push_str(&raw);
                    i = end + 1;
                    continue;
                }
            }

            // 普通文本：收集到下一个反引号或空格
            let mut segment = String::new();
            while i < chars.len() && chars[i] != '`' && chars[i] != ' ' {
                segment.push(chars[i]);
                i += 1;
            }

            if !segment.is_empty() {
                let converted = self.convert_fragment(&segment);
                if !result.is_empty() && !result.ends_with(' ') {
                    result.push(' ');
                }
                result.push_str(&converted);
            }

            // 跳过空格（保留一个空格）
            if i < chars.len() && chars[i] == ' ' {
                if !result.is_empty() && !result.ends_with(' ') {
                    result.push(' ');
                }
                i += 1;
            }
        }

        result
    }

    /// 转换并保留原始分隔（空格分隔的音节）
    ///
    /// 与 convert 不同，此方法对每个空格分隔的片段强制尝试转换，
    /// 即使片段无法被完整解析为双拼，也会尽可能转换能解析的部分。
    #[allow(dead_code)]
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
