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
        // 先检查条件韵母映射（非默认的优先）
        for (prefixes, k, fin) in &self.data.conditional_finals {
            if *k == key && !prefixes.is_empty() {
                if prefixes.chars().any(|p| initial.starts_with(p)) {
                    return Some(fin.clone());
                }
            }
        }

        // 再检查条件韵母的默认映射
        for (prefixes, k, fin) in &self.data.conditional_finals {
            if *k == key && prefixes.is_empty() {
                return Some(fin.clone());
            }
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
            let final_str = if let Some(s) = self.get_final(&initial, final_char) {
                s
            } else if let Some(s) = self.data.finals.get(&final_char) {
                s.clone()
            } else {
                final_char.to_string()
            };

            return Some(initial + &final_str);
        }

        None
    }

    /// 转换一段双拼文本为全拼
    ///
    /// 将连续的双拼编码转换为全拼，每两个字符一组
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
