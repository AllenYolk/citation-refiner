use scraper::{Html, Selector};

pub fn get_html_text(url: &str) -> String {
    reqwest::blocking::get(url).unwrap()
        .text().unwrap()
}

pub fn select_html_text(raw_html: &str, raw_selector: &str) -> Vec<String> {
    let html = if raw_html.starts_with("<!DOCTYPE html>") {
        Html::parse_document(raw_html)
    } else {
        Html::parse_fragment(raw_html)
    };
    let selector = Selector::parse(raw_selector).unwrap();

    let mut ret: Vec<String> = Vec::new();
    for e in html.select(&selector) {
        ret.push(e.text().collect());
    };
    ret
}

pub fn select_html_attribute(raw_html: &str, raw_selector: &str, attribute_name: &str) -> Vec<String> {
    let html = if raw_html.starts_with("<!DOCTYPE html>") {
        Html::parse_document(raw_html)
    } else {
        Html::parse_fragment(raw_html)
    };
    let selector = Selector::parse(raw_selector).unwrap();

    let mut ret: Vec<String> = Vec::new();
    for e in html.select(&selector) {
        let attr = e.value().attr(attribute_name).unwrap();
        ret.push(String::from(attr));
    };
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reqwest_dblp_test() {
        let url = "https://dblp.org";
        let html_text = get_html_text(url);
        println!("{}", html_text);

        assert_ne!(html_text.len(), 0);
    }

    #[test]
    fn select_html_text_test() {
        let raw_html = "<div> shit, <a href=\"fuck\">derderder</a>, <a href=\"holy\">bbb</a>, haha</div>";
        let ans = select_html_text(&raw_html, "div a");

        assert_eq!(ans[0], "derderder");
        assert_eq!(ans[1], "bbb");
    }

    #[test]
    fn select_html_attribute_test() {
        let raw_html = "<!DOCTYPE html> <body><div> shit, <a href=\"fuck\">derderder</a>, <a href=\"holy\">bbb</a>, haha</div></body>";
        let ans = select_html_attribute(&raw_html, "div a", "href");

        assert_eq!(ans[0], "fuck");
        assert_eq!(ans[1], "holy");
    }
}
