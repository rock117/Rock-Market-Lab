use itertools::Itertools;
use lopdf::Document;

/// 读取pdf文件，每页之间用换行符分隔
pub fn read_pdf_text(path: &str) -> anyhow::Result<String> {
    let document = Document::load(path)?;
    let pages = document.get_pages();
    let mut texts = Vec::new();

    for (i, _) in pages.iter().enumerate() {
        let page_number = (i + 1) as u32;
        let text = document.extract_text(&[page_number])?;
        texts.push(text);
    }
    Ok(texts.iter().join("\n"))
}

#[cfg(test)]
mod tests {
    use super::read_pdf_text;

    #[test]
    fn test_read_pdf() {
        let content = read_pdf_text(
            r#"C:\rock\doc\book\101 Google Tricks Tips and Hacks () (Z-Library).pdf"#,
        )
        .unwrap();
        println!("{content}");
    }
}
