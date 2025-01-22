pub struct HtmlParser {
    html: String,
}

impl HtmlParser {
    pub fn new(html: String) -> HtmlParser {
        HtmlParser { html }
    }
    pub fn select(selector: &str) -> Vec<Element> {
        todo!()
    }
}

pub struct Element {}
