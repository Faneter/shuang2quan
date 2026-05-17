use shuang2quan::converter::ShuangPinConverter;
use shuang2quan::scheme::SchemeData;

#[test]
fn test_xiaohe_shuang() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    // 小鹤双拼: sh->u, uang->l
    assert_eq!(converter.convert_syllable("ul"), Some("shuang".to_string()));
}

#[test]
fn test_xiaohe_jiang() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    // 小鹤双拼: j->j, iang->l
    assert_eq!(converter.convert_syllable("jl"), Some("jiang".to_string()));
}

#[test]
fn test_xiaohe_qiong() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    assert_eq!(converter.convert_syllable("qs"), Some("qiong".to_string()));
}

#[test]
fn test_xiaohe_song() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    assert_eq!(converter.convert_syllable("ss"), Some("song".to_string()));
}

#[test]
fn test_xiaohe_pin() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    assert_eq!(converter.convert_syllable("pb"), Some("pin".to_string()));
}

#[test]
fn test_xiaohe_full() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    assert_eq!(converter.convert("ul pb"), "shuang pin");
}

#[test]
fn test_microsoft_shuang() {
    let converter = ShuangPinConverter::from_name("microsoft").unwrap();
    assert_eq!(converter.convert_syllable("ed"), Some("shuang".to_string()));
}

#[test]
fn test_microsoft_jiang() {
    let converter = ShuangPinConverter::from_name("microsoft").unwrap();
    assert_eq!(converter.convert_syllable("jd"), Some("jiang".to_string()));
}

#[test]
fn test_microsoft_pin() {
    let converter = ShuangPinConverter::from_name("microsoft").unwrap();
    assert_eq!(converter.convert_syllable("pb"), Some("pin".to_string()));
}

#[test]
fn test_scheme_from_name() {
    assert!(ShuangPinConverter::from_name("xiaohe").is_some());
    assert!(ShuangPinConverter::from_name("microsoft").is_some());
    assert!(ShuangPinConverter::from_name("sougou").is_some());
    assert!(ShuangPinConverter::from_name("unknown").is_none());
}

#[test]
fn test_empty_input() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    assert_eq!(converter.convert(""), "");
}

#[test]
fn test_single_char() {
    let converter = ShuangPinConverter::from_name("xiaohe").unwrap();
    assert_eq!(converter.convert_syllable("a"), Some("a".to_string()));
}

#[test]
fn test_custom_scheme_from_str() {
    let config = r#"
# 自定义双拼方案
initial:v=zh
initial:i=ch
initial:u=sh
final:a=a
final:b=in
final:d=iang
final:s=ong
conditional:jqx,d=iang
conditional:y,d=iang
conditional:,d=uang
"#;

    let data = SchemeData::from_str(config).unwrap();
    let converter = ShuangPinConverter::new(data);

    assert_eq!(converter.convert_syllable("ud"), Some("shuang".to_string()));
    assert_eq!(converter.convert_syllable("jd"), Some("jiang".to_string()));
    assert_eq!(converter.convert_syllable("pb"), Some("pin".to_string()));
}

#[test]
fn test_custom_scheme_builder() {
    let data = SchemeData::new()
        .add_initial('v', "zh")
        .add_initial('i', "ch")
        .add_initial('u', "sh")
        .add_final('a', "a")
        .add_final('b', "in")
        .add_final('d', "iang")
        .add_final('s', "ong")
        .add_conditional_final("jqx", 'd', "iang")
        .add_conditional_final("y", 'd', "iang")
        .add_conditional_final("", 'd', "uang");

    let converter = ShuangPinConverter::new(data);
    assert_eq!(converter.convert_syllable("ud"), Some("shuang".to_string()));
    assert_eq!(converter.convert_syllable("jd"), Some("jiang".to_string()));
}

#[test]
fn test_invalid_config() {
    let config = "invalid_line";
    assert!(SchemeData::from_str(config).is_err());
}
