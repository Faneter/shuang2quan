# shuang2quan

一个支持多种双拼方案的双拼转全拼命令行工具，采用 Rust 编写。

## 功能特性

- **内置 6 种双拼方案**：小鹤、微软、搜狗、自然码、智能 ABC、紫光
- **自定义方案**：通过配置文件或代码 API 定义任意双拼方案
- **条件韵母支持**：自动处理 `iang/uang`、`ong/iong` 等共享键位
- **零依赖**：纯 Rust 标准库实现，无需额外依赖

## 安装

```bash
git clone https://github.com/yourname/shuang2quan.git
cd shuang2quan
cargo build --release
```

编译后的可执行文件位于 `target/release/shuang2quan`。

## 快速开始

```bash
# 使用默认方案（小鹤双拼）
shuang2quan "ud pb"
# 输出: shuang pin

# 指定内置方案
shuang2quan -s microsoft "ed pb"
# 输出: shuang pin

# 查看所有内置方案
shuang2quan --list
```

## 命令行用法

```
用法: shuang2quan [选项] <双拼文本>

选项:
  -s, --scheme <方案>   指定内置双拼方案 (默认: xiaohe)
  -c, --config <文件>   从配置文件加载自定义方案
  -h, --help            显示帮助信息
  -l, --list            列出所有支持的双拼方案
```

## 自定义双拼方案

### 配置文件格式

创建文本文件（如 `myscheme.txt`）：

```text
# 声母映射: initial:键=全拼声母
initial:v=zh
initial:i=ch
initial:u=sh

# 韵母映射: final:键=全拼韵母
final:a=a
final:b=in
final:c=ao
final:d=iang

# 条件韵母映射: conditional:声母前缀,键=韵母
# 用于处理 iang/uang、ong/iong 等共享键位
conditional:jqx,d=iang    # j/q/x 声母 + d 键 → iang
conditional:y,d=iang      # y 声母 + d 键 → iang
conditional:,d=uang       # 其他声母 + d 键 → uang（默认）
```

### 使用自定义方案

```bash
shuang2quan -c myscheme.txt "ud pb"
```

## 作为库使用

在 `Cargo.toml` 中添加：

```toml
[dependencies]
shuang2quan = { path = "path/to/shuang2quan" }
```

### 使用内置方案

```rust
use shuang2quan::converter::ShuangPinConverter;

let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
let result = converter.convert("ud pb");
assert_eq!(result, "shuang pin");
```

### 构建自定义方案

```rust
use shuang2quan::{converter::ShuangPinConverter, scheme::SchemeData};

let data = SchemeData::new()
    .add_initial('v', "zh")
    .add_initial('i', "ch")
    .add_initial('u', "sh")
    .add_final('a', "a")
    .add_final('b', "in")
    .add_final('d', "iang")
    .add_conditional_final("jqx", 'd', "iang")
    .add_conditional_final("y", 'd', "iang")
    .add_conditional_final("", 'd', "uang");

let converter = ShuangPinConverter::new(data);
assert_eq!(converter.convert_syllable("ud"), Some("shuang".to_string()));
```

### 从字符串解析方案

```rust
use shuang2quan::scheme::SchemeData;

let config = r#"
initial:v=zh
final:b=in
conditional:,d=uang
"#;

let data = SchemeData::from_str(config).unwrap();
```

## 内置方案键位对照

### 小鹤双拼

| 键位 | 声母 |   韵母    |
| :--: | :--: | :-------: |
|  a   |  —   |     a     |
|  b   |  b   |    in     |
|  c   |  c   |    ao     |
|  d   |  d   |    ai     |
|  e   |  —   |     e     |
|  f   |  f   |    en     |
|  g   |  g   |    eng    |
|  h   |  h   |    ang    |
|  i   |  ch  |     i     |
|  j   |  j   |    an     |
|  k   |  k   |  uai/ing  |
|  l   |  l   | iang/uang |
|  m   |  m   |    ian    |
|  n   |  n   |    iao    |
|  o   |  —   |   o/uo    |
|  p   |  p   |    ie     |
|  q   |  q   |    iu     |
|  r   |  r   |  uan/van  |
|  s   |  s   | ong/iong  |
|  t   |  t   |   ve/ue   |
|  u   |  sh  |     u     |
|  v   |  zh  |   ui/v    |
|  w   |  w   |    ei     |
|  x   |  x   |   ia/ua   |
|  y   |  y   |   un/vn   |
|  z   |  z   |    ou     |

> 注：斜杠表示条件韵母，根据声母自动选择。例如 `l` 键在 `j/q/x/y` 声母后为 `iang`，其他声母后为 `uang`。

## 项目结构

```
shuang2quan/
├── src/
│   ├── lib.rs          # 库入口
│   ├── main.rs         # 命令行入口
│   ├── scheme.rs       # 方案数据结构（内置方案 + 自定义解析）
│   └── converter.rs    # 转换核心逻辑
├── tests/
│   └── integration_tests.rs
└── Cargo.toml
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行并显示输出
cargo test -- --nocapture
```

## 许可证

MIT
