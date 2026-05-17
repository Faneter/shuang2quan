use shuang2quan::converter::ShuangPinConverter;
use shuang2quan::scheme::ShuangPinScheme;

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
    // "ud pb" -> "shuang pin"
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
