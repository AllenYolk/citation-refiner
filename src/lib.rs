mod clipboard;
mod html;

use clap::ValueEnum;
use html::*;
use clipboard::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Website {
    Dblp,
}

fn get_direct_urls_dblp(query: &str) -> Vec<String>{
    let query_in_url = query.replace(" ", "%20");
    let base_url = format!("https://dblp.org/search?q={}", query_in_url);
    println!("url: {}", &base_url);

    let raw_html = get_html_text(&base_url);
    select_html_attribute(&raw_html, "nav.publ > ul > li > div > a[rel]", "href")
}

fn get_direct_urls(query: &str, website: Website) -> Vec<String>{
    match website {
        Website::Dblp => get_direct_urls_dblp(query),
        // _ => panic!("Website {:?} is not supported!", website)
    }
}

pub fn run(query: &str, website: Website, n_considered: usize) {
    let urls = get_direct_urls(query, website);

    for (i, url) in urls.iter().enumerate() {
        if i >= n_considered {
            break;
        }

        let raw_html = get_html_text(url);
        println!("{}", raw_html);
        let bibtex = select_html_text(&raw_html, "#bibtex-section > pre.select-on-click");

        assert_eq!(bibtex.len(), 1);
        copy_text(&bibtex[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_urls_test() {
        let query = "Attention is All you Need, vaswani";

        let res_dblp = get_direct_urls(query, Website::Dblp);
        assert_eq!(res_dblp[0], "https://dblp.org/rec/conf/nips/VaswaniSPUJGKP17.html?view=bibtex");
        assert_eq!(res_dblp[1], "https://dblp.org/rec/journals/corr/VaswaniSPUJGKP17.html?view=bibtex");
    }

    #[test]
    fn run_test() {
        let query = "Attention is All you Need, vaswani";
        run(query, Website::Dblp, 1);
        assert!(get_copied_text().starts_with("@inproceedings{DBLP:conf/nips/VaswaniSPUJGKP17,"));

        run(query, Website::Dblp, 2);
        assert!(get_copied_text().starts_with("@article{DBLP:journals/corr/VaswaniSPUJGKP17,"));

        run(query, Website::Dblp, 114514);
        assert!(get_copied_text().starts_with("@article{DBLP:journals/corr/VaswaniSPUJGKP17,"));
    }
}