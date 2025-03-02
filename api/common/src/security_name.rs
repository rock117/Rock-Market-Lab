use pinyin::ToPinyin;

pub fn get_security_pinyin(text: &str) -> String {
    let pinyin: String = text
        .to_pinyin() // 转换为拼音迭代器
        .filter_map(|pinyin| pinyin) // 过滤掉非汉字字符（返回 None 的情况）
        .map(|pinyin| pinyin.first_letter()) // 获取拼音的首字母
        .collect(); // 收集为字符串
    pinyin
}

mod tests {
    use crate::security_name::get_security_pinyin;

    #[test]
    fn test_get_security_pinyin() {
        let text = "360你好，世界！";
        let res = get_security_pinyin(text);
        println!("res = {}", res)
    }
}



//
// fn main() {
//     let text = "你好，世界！";
//
//     // 将汉字转换为拼音，并提取首字母
//     let initials: String = text
//         .to_pinyin() // 转换为拼音迭代器
//         .filter_map(|pinyin| pinyin) // 过滤掉非汉字字符（返回 None 的情况）
//         .map(|pinyin| pinyin.first_letter()) // 获取拼音的首字母
//         .collect(); // 收集为字符串
//
//     println!("拼音首字母: {}", initials);
// }