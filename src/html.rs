use scraper::{Html, Selector};

pub fn get_html_text(url: &str) -> Result<String, ()> {
    reqwest::blocking::get(url).unwrap().text().map_err(|_| ())
}

pub fn select_html_text(raw_html: &str, raw_selector: &str) -> Result<Vec<String>, ()> {
    let html = if raw_html.starts_with("<!DOCTYPE html>") {
        Html::parse_document(raw_html)
    } else {
        Html::parse_fragment(raw_html)
    };
    let selector = Selector::parse(raw_selector).map_err(|_| ())?;

    let mut ret: Vec<String> = Vec::new();
    for e in html.select(&selector) {
        ret.push(e.text().collect());
    }
    Ok(ret)
}

pub fn select_html_attribute(
    raw_html: &str,
    raw_selector: &str,
    attribute_name: &str,
) -> Result<Vec<String>, ()> {
    let html = if raw_html.starts_with("<!DOCTYPE html>") {
        Html::parse_document(raw_html)
    } else {
        Html::parse_fragment(raw_html)
    };
    let selector = Selector::parse(raw_selector).map_err(|_| ())?;

    let mut ret: Vec<String> = Vec::new();
    for e in html.select(&selector) {
        let Some(attr) = e.value().attr(attribute_name) else {
            continue;
        };
        ret.push(String::from(attr));
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reqwest_dblp_test() {
        let url = "https://dblp.org";
        let html_text = get_html_text(url).unwrap();
        println!("{}", html_text);

        assert_ne!(html_text.len(), 0);
    }

    #[test]
    fn select_html_text_test() {
        let raw_html = "<div> shit, <a href=\"fuck\">derderder\nderderder</a>, <a href=\"holy\">bbb</a>, haha</div>";
        let ans = select_html_text(&raw_html, "div a").unwrap();

        assert_eq!(ans[0], "derderder\nderderder");
        assert_eq!(ans[1], "bbb");
    }

    #[test]
    fn select_html_attribute_test() {
        let raw_html = "<!DOCTYPE html> <body><div> shit, <a href=\"fuck\">derderder</a>, <a href=\"holy\">bbb</a>, haha</div></body>";
        let ans = select_html_attribute(&raw_html, "div a", "href").unwrap();

        assert_eq!(ans[0], "fuck");
        assert_eq!(ans[1], "holy");
    }
}
