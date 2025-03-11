use pinyin::ToPinyin;

pub fn get_security_pinyin(text: &str) -> String {
    let mut result = String::new();
    let words = split_chinese_sequence(text);
    for word in words {
        if word.is_zh {
            let pinyin: String = word.text.as_str()
                .to_pinyin() // 转换为拼音迭代器
                .filter_map(|pinyin| pinyin) // 过滤掉非汉字字符（返回 None 的情况）
                .map(|pinyin| pinyin.first_letter()) // 获取拼音的首字母
                .collect(); // 收集为字符串
            result.push_str(&pinyin);
        } else {
            result.push_str(&word.text);
        }
    }

    result
}

#[derive(Debug, Clone)]
struct Word {
    text: String,
    is_zh: bool
}

/// 将字符串分割成汉字和非汉字的序列
/// 连续的汉字或非汉字会被组合在一起
/// 例如: "abc中3国家d" -> ["abc", "中", "3", "国家", "d"]
fn split_chinese_sequence(input: &str) -> Vec<Word> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut last_is_chinese = false;

    for c in input.chars() {
        let is_chinese = ('\u{4e00}'..='\u{9fff}').contains(&c);

        if current.is_empty() {
            current.push(c);
            last_is_chinese = is_chinese;
        } else if is_chinese == last_is_chinese {
            current.push(c);
        } else {
            result.push(Word {
                text: current,
                is_zh: last_is_chinese
            });
            current = String::from(c);
            last_is_chinese = is_chinese;
        }
    }

    if !current.is_empty() {
        result.push(Word {
            text: current,
            is_zh: last_is_chinese
        });
    }

    result
}
mod tests {
    use crate::security_name::{get_security_pinyin, split_chinese_sequence};

    #[test]
    fn test_get_security_pinyin() {
        let text = "360你好，世界！";
        let res = get_security_pinyin(text);
        println!("res = {}", res)
    }

    #[test]
    fn test_split_chinese_sequence() {
        let input = "abc中3国家d";
        let result = split_chinese_sequence(input);
        println!("{:?}", result);
     //   assert_eq!(result, vec!["abc", "中", "3", "国家", "d"]);

        let input2 = "Hello你好World世界123";
        let result2 = split_chinese_sequence(input2);
        println!("{:?}", result2);
       // assert_eq!(result2, vec!["Hello", "你好", "World", "世界", "123"]);
    }

}