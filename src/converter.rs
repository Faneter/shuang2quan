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

    /// 检查一个字符是否是有效的双拼键位（声母或韵母键）
    fn is_valid_key(&self, c: char) -> bool {
        // 基本声母键
        if ('a'..='z').contains(&c) {
            return true;
        }
        // 特殊符号键（部分方案使用）
        matches!(c, ';' | '\'')
    }

    /// 检查一个字符是否是标点符号
    fn is_punctuation(&self, c: char) -> bool {
        matches!(
            c,
            ',' | '.'
                | '!'
                | '?'
                | ':'
                | ';'
                | '"'
                | '\''
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '<'
                | '>'
                | '/'
                | '\\'
                | '|'
                | '@'
                | '#'
                | '$'
                | '%'
                | '^'
                | '&'
                | '*'
                | '+'
                | '='
                | '~'
                | '`'
                | '·'
                | '，'
                | '。'
                | '！'
                | '？'
                | '：'
                | '；'
                | '「'
                | '」'
                | '【'
                | '】'
                | '（'
                | '）'
                | '《'
                | '》'
                | '、'
                | '—'
                | '…'
        )
    }

    /// 判断一段文本是否"看起来像"双拼编码
    ///
    /// 启发式规则：
    /// 1. 只包含小写字母和少量特殊符号
    /// 2. 不包含大写字母（英文单词通常有大写）
    /// 3. 不包含连续的辅音组合（如 "rst", "str", "ndows" 等英文常见组合）
    fn looks_like_shuangpin(&self, segment: &str) -> bool {
        if segment.is_empty() {
            return false;
        }

        // 如果包含大写字母，一定不是双拼
        if segment.chars().any(|c| c.is_uppercase()) {
            return false;
        }

        // 如果包含非双拼键位的字符（如数字等），不是双拼
        if !segment.chars().all(|c| self.is_valid_key(c)) {
            return false;
        }

        // 启发式：双拼编码中，连续的辅音字母组合应该很少见
        // 英文单词常见的三辅音组合（如 rst, str, ndw 等）不太会出现在双拼中
        let consonant_clusters = ["rst", "str", "ndw", "rld", "cks", "tch", "nch"];
        for cluster in &consonant_clusters {
            if segment.contains(cluster) {
                return false;
            }
        }

        true
    }

    /// 检查一个拼音是否"看起来有效"
    ///
    /// 用于过滤掉明显不是有效拼音的结果（如 "sve", "rld" 等）
    fn looks_like_valid_pinyin(&self, pinyin: &str) -> bool {
        // 常见无效拼音模式
        let invalid_patterns = ["sve", "rld", "ndw", "fwe", "bve", "pve", "mve"];
        for pattern in &invalid_patterns {
            if pinyin.contains(pattern) {
                return false;
            }
        }
        true
    }

    /// 尝试将一段文本完整解析为双拼
    ///
    /// 如果文本能被完全解析为双拼音节（每两个字符一组），返回全拼结果；
    /// 否则返回 None，表示这段文本不是双拼编码。
    fn try_convert_segment(&self, segment: &str) -> Option<String> {
        if segment.is_empty() {
            return Some(String::new());
        }

        // 首先判断这段文本是否"看起来像"双拼
        if !self.looks_like_shuangpin(segment) {
            return None;
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

        // 验证解析结果是否都是有效拼音
        if !self.looks_like_valid_pinyin(&result) {
            return None;
        }

        Some(result)
    }

    /// 检查片段是否是邮箱、URL 等特殊格式
    fn is_special_format(&self, fragment: &str) -> bool {
        // 邮箱格式
        if fragment.contains('@') && fragment.contains('.') {
            return true;
        }
        // URL 常见后缀
        if fragment.contains("://") || fragment.contains("www.") {
            return true;
        }
        // 域名后缀
        let domain_suffixes = [".com", ".cn", ".org", ".net", ".io", ".dev", ".rs"];
        for suffix in &domain_suffixes {
            if fragment.ends_with(suffix) {
                return true;
            }
        }
        false
    }

    /// 处理一个片段，按标点符号分割后分别转换
    fn convert_fragment(&self, fragment: &str) -> String {
        if fragment.is_empty() {
            return String::new();
        }

        // 如果是特殊格式（邮箱、URL 等），直接保留
        if self.is_special_format(fragment) {
            return fragment.to_string();
        }

        let mut result = String::new();
        let mut current_word = String::new();

        for c in fragment.chars() {
            if self.is_punctuation(c) {
                // 遇到标点，先转换当前累积的单词
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
    /// 按空格分割文本，对每个片段判断是否为双拼编码：
    /// - 如果能完整解析为双拼，则转换
    /// - 否则保留原样
    pub fn convert(&self, input: &str) -> String {
        let segments: Vec<&str> = input.split(' ').collect();
        let mut result = Vec::new();

        for segment in segments {
            result.push(self.convert_fragment(segment));
        }

        result.join(" ")
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
