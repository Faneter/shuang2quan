use std::collections::HashMap;

/// 双拼方案数据：包含声母映射和韵母映射
/// 对于共享键位的韵母（如 iang/uang），使用需要根据声母区分的映射
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeData {
    pub initials: HashMap<char, String>,
    pub finals: HashMap<char, String>,
    /// 条件韵母映射: (声母前缀, 键位, 韵母)
    /// 用于处理 iang/uang、ong/iong 等共享键位的情况
    pub conditional_finals: Vec<(String, char, String)>,
}

impl Default for SchemeData {
    fn default() -> Self {
        SchemeData {
            initials: HashMap::new(),
            finals: HashMap::new(),
            conditional_finals: Vec::new(),
        }
    }
}

impl SchemeData {
    /// 创建空的方案数据
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加声母映射
    pub fn add_initial(mut self, key: char, value: &str) -> Self {
        self.initials.insert(key, value.to_string());
        self
    }

    /// 添加韵母映射
    pub fn add_final(mut self, key: char, value: &str) -> Self {
        self.finals.insert(key, value.to_string());
        self
    }

    /// 添加条件韵母映射
    pub fn add_conditional_final(mut self, prefixes: &str, key: char, value: &str) -> Self {
        self.conditional_finals
            .push((prefixes.to_string(), key, value.to_string()));
        self
    }

    /// 从字符串解析方案数据
    ///
    /// 格式:
    /// ```text
    /// # 声母映射
    /// initial:v=zh
    /// initial:i=ch
    /// initial:u=sh
    ///
    /// # 韵母映射
    /// final:a=a
    /// final:b=in
    /// final:d=iang
    ///
    /// # 条件韵母映射 (声母前缀:键位=韵母)
    /// conditional:jqx,d=iang
    /// conditional:y,d=iang
    /// conditional:,d=uang
    /// ```
    pub fn from_str(content: &str) -> Result<Self, String> {
        let mut data = SchemeData::new();

        for (line_no, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(kv) = line.strip_prefix("initial:") {
                let parts: Vec<&str> = kv.splitn(2, '=').collect();
                if parts.len() != 2 {
                    return Err(format!("第 {} 行: 无效的声母映射 '{}'", line_no + 1, line));
                }
                let key = parts[0].trim();
                if key.chars().count() != 1 {
                    return Err(format!(
                        "第 {} 行: 声母键必须是单个字符 '{}'",
                        line_no + 1,
                        key
                    ));
                }
                data.initials
                    .insert(key.chars().next().unwrap(), parts[1].trim().to_string());
            } else if let Some(kv) = line.strip_prefix("final:") {
                let parts: Vec<&str> = kv.splitn(2, '=').collect();
                if parts.len() != 2 {
                    return Err(format!("第 {} 行: 无效的韵母映射 '{}'", line_no + 1, line));
                }
                let key = parts[0].trim();
                if key.chars().count() != 1 {
                    return Err(format!(
                        "第 {} 行: 韵母键必须是单个字符 '{}'",
                        line_no + 1,
                        key
                    ));
                }
                data.finals
                    .insert(key.chars().next().unwrap(), parts[1].trim().to_string());
            } else if let Some(kv) = line.strip_prefix("conditional:") {
                let parts: Vec<&str> = kv.splitn(2, '=').collect();
                if parts.len() != 2 {
                    return Err(format!(
                        "第 {} 行: 无效的条件韵母映射 '{}'",
                        line_no + 1,
                        line
                    ));
                }
                let cond_parts: Vec<&str> = parts[0].split(',').collect();
                if cond_parts.len() != 2 {
                    return Err(format!(
                        "第 {} 行: 条件格式必须是 'prefixes,key' '{}'",
                        line_no + 1,
                        parts[0]
                    ));
                }
                let prefixes = cond_parts[0].trim();
                let key = cond_parts[1].trim();
                if key.chars().count() != 1 {
                    return Err(format!(
                        "第 {} 行: 条件键必须是单个字符 '{}'",
                        line_no + 1,
                        key
                    ));
                }
                data.conditional_finals.push((
                    prefixes.to_string(),
                    key.chars().next().unwrap(),
                    parts[1].trim().to_string(),
                ));
            } else {
                return Err(format!("第 {} 行: 未知的行格式 '{}'", line_no + 1, line));
            }
        }

        Ok(data)
    }

    /// 内置方案：小鹤双拼
    pub fn xiaohe() -> Self {
        SchemeData::new()
            // 特殊声母
            .add_initial('v', "zh")
            .add_initial('i', "ch")
            .add_initial('u', "sh")
            // 韵母
            .add_final('a', "a")
            .add_final('o', "o")
            .add_final('e', "e")
            .add_final('i', "i")
            .add_final('u', "u")
            .add_final('v', "ui")
            .add_final('b', "in")
            .add_final('c', "ao")
            .add_final('f', "en")
            .add_final('g', "eng")
            .add_final('h', "ang")
            .add_final('j', "an")
            .add_final('k', "ing")
            .add_final('l', "ai")
            .add_final('m', "ian")
            .add_final('n', "iao")
            .add_final('p', "ie")
            .add_final('q', "iu")
            .add_final('r', "uan")
            .add_final('t', "ue")
            .add_final('w', "ei")
            .add_final('x', "ia")
            .add_final('y', "un")
            .add_final('z', "ou")
            // 条件韵母
            .add_conditional_final("jqx", 'd', "iang")
            .add_conditional_final("y", 'd', "iang")
            .add_conditional_final("", 'd', "uang")
            .add_conditional_final("jqx", 's', "iong")
            .add_conditional_final("y", 's', "iong")
            .add_conditional_final("", 's', "ong")
    }

    /// 内置方案：微软双拼
    pub fn microsoft() -> Self {
        SchemeData::new()
            // 特殊声母
            .add_initial('a', "zh")
            .add_initial('o', "ch")
            .add_initial('e', "sh")
            // 韵母
            .add_final('a', "a")
            .add_final('o', "o")
            .add_final('e', "e")
            .add_final('i', "i")
            .add_final('u', "u")
            .add_final('v', "ui")
            .add_final('b', "in")
            .add_final('c', "ao")
            .add_final('f', "en")
            .add_final('g', "eng")
            .add_final('h', "ang")
            .add_final('j', "an")
            .add_final('k', "ing")
            .add_final('l', "ai")
            .add_final('m', "ian")
            .add_final('n', "iao")
            .add_final('p', "ie")
            .add_final('q', "iu")
            .add_final('r', "uan")
            .add_final('t', "ue")
            .add_final('w', "ei")
            .add_final('x', "ia")
            .add_final('y', "un")
            .add_final('z', "ou")
            .add_final(';', "ing")
            // 条件韵母
            .add_conditional_final("jqx", 'd', "iang")
            .add_conditional_final("y", 'd', "iang")
            .add_conditional_final("", 'd', "uang")
            .add_conditional_final("jqx", 's', "iong")
            .add_conditional_final("y", 's', "iong")
            .add_conditional_final("", 's', "ong")
    }

    /// 内置方案：搜狗双拼（与微软相同）
    pub fn sougou() -> Self {
        Self::microsoft()
    }

    /// 内置方案：自然码双拼
    pub fn ziranma() -> Self {
        SchemeData::new()
            // 特殊声母
            .add_initial('v', "zh")
            .add_initial('i', "ch")
            .add_initial('u', "sh")
            // 韵母
            .add_final('a', "a")
            .add_final('o', "o")
            .add_final('e', "e")
            .add_final('i', "i")
            .add_final('u', "u")
            .add_final('v', "ui")
            .add_final('b', "in")
            .add_final('c', "ao")
            .add_final('f', "en")
            .add_final('g', "eng")
            .add_final('h', "ang")
            .add_final('j', "an")
            .add_final('k', "ing")
            .add_final('l', "ai")
            .add_final('m', "ian")
            .add_final('n', "iao")
            .add_final('p', "ie")
            .add_final('q', "iu")
            .add_final('r', "uan")
            .add_final('t', "ue")
            .add_final('w', "ei")
            .add_final('x', "ia")
            .add_final('y', "un")
            .add_final('z', "ou")
            .add_final(';', "ing")
            // 条件韵母
            .add_conditional_final("jqx", 'd', "iang")
            .add_conditional_final("y", 'd', "iang")
            .add_conditional_final("", 'd', "uang")
            .add_conditional_final("jqx", 's', "iong")
            .add_conditional_final("y", 's', "iong")
            .add_conditional_final("", 's', "ong")
    }

    /// 内置方案：智能ABC双拼
    pub fn zhinengabc() -> Self {
        SchemeData::new()
            // 特殊声母
            .add_initial('a', "zh")
            .add_initial('e', "ch")
            .add_initial('v', "sh")
            // 韵母
            .add_final('a', "a")
            .add_final('o', "o")
            .add_final('e', "e")
            .add_final('i', "i")
            .add_final('u', "u")
            .add_final('v', "ui")
            .add_final('b', "in")
            .add_final('c', "ao")
            .add_final('f', "en")
            .add_final('g', "eng")
            .add_final('h', "ang")
            .add_final('j', "an")
            .add_final('k', "ing")
            .add_final('l', "ai")
            .add_final('m', "ian")
            .add_final('n', "iao")
            .add_final('p', "ie")
            .add_final('q', "iu")
            .add_final('r', "uan")
            .add_final('t', "ue")
            .add_final('w', "ei")
            .add_final('x', "ia")
            .add_final('y', "un")
            .add_final('z', "ou")
            .add_final('\'', "ing")
            // 条件韵母
            .add_conditional_final("jqx", 'd', "iang")
            .add_conditional_final("y", 'd', "iang")
            .add_conditional_final("", 'd', "uang")
            .add_conditional_final("jqx", 's', "iong")
            .add_conditional_final("y", 's', "iong")
            .add_conditional_final("", 's', "ong")
    }

    /// 内置方案：紫光双拼
    pub fn ziguang() -> Self {
        SchemeData::new()
            // 特殊声母
            .add_initial('u', "sh")
            .add_initial('a', "ch")
            .add_initial('i', "zh")
            // 韵母
            .add_final('a', "a")
            .add_final('o', "o")
            .add_final('e', "e")
            .add_final('i', "i")
            .add_final('u', "u")
            .add_final('v', "ui")
            .add_final('b', "in")
            .add_final('c', "ao")
            .add_final('f', "en")
            .add_final('g', "eng")
            .add_final('h', "ang")
            .add_final('j', "an")
            .add_final('k', "ing")
            .add_final('l', "ai")
            .add_final('m', "ian")
            .add_final('n', "iao")
            .add_final('p', "ie")
            .add_final('q', "iu")
            .add_final('r', "uan")
            .add_final('t', "ue")
            .add_final('w', "ei")
            .add_final('x', "ia")
            .add_final('y', "un")
            .add_final('z', "ou")
            .add_final(';', "ing")
            // 条件韵母
            .add_conditional_final("jqx", 'd', "iang")
            .add_conditional_final("y", 'd', "iang")
            .add_conditional_final("", 'd', "uang")
            .add_conditional_final("jqx", 's', "iong")
            .add_conditional_final("y", 's', "iong")
            .add_conditional_final("", 's', "ong")
    }

    /// 根据名称获取内置方案
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "xiaohe" => Some(Self::xiaohe()),
            "microsoft" => Some(Self::microsoft()),
            "sougou" | "sogou" => Some(Self::sougou()),
            "ziranma" => Some(Self::ziranma()),
            "zhinengabc" => Some(Self::zhinengabc()),
            "ziguang" => Some(Self::ziguang()),
            _ => None,
        }
    }

    /// 获取所有内置方案名称
    pub fn builtin_names() -> &'static [&'static str] {
        &[
            "xiaohe",
            "microsoft",
            "sougou",
            "ziranma",
            "zhinengabc",
            "ziguang",
        ]
    }
}
