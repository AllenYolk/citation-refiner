mod bibtex;
mod clipboard;
mod html;

use bibtex::*;
use clap::ValueEnum;
use clipboard::*;
use html::*;
use std::io::{stdin, stdout, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Website {
    Dblp,
}

pub enum RunError {
    GetUrlError,
    GetBibtexError { url: String },
    MultipleBibtexError { url: String },
    CopyToClipboardError,
}

fn get_direct_urls_dblp(query: &str) -> Result<Vec<String>, ()> {
    let query_in_url = query.replace(" ", "%20");
    let base_url = format!("https://dblp.org/search?q={}", query_in_url);
    println!("url: {}", &base_url);

    let raw_html = get_html_text(&base_url)?;
    select_html_attribute(&raw_html, "nav.publ > ul > li > div > a[rel]", "href")
}

fn get_direct_urls(query: &str, website: Website) -> Result<Vec<String>, ()> {
    match website {
        Website::Dblp => get_direct_urls_dblp(query),
        // _ => panic!("Website {:?} is not supported!", website),
    }
}

fn get_bibtex_dblp(url: &str) -> Result<Vec<String>, ()> {
    let raw_html = get_html_text(&url)?;
    select_html_text(&raw_html, "#bibtex-section > pre.select-on-click")
}

fn get_bibtex(url: &str, website: Website) -> Result<Vec<String>, ()> {
    match website {
        Website::Dblp => get_bibtex_dblp(url),
        // _ => panic!("Website {:?} is not supported!", website),
    }
}

pub fn run(
    query: &str,
    website: Website,
    n_considered: usize,
    ignore_preprint: bool,
    full_bibtex: bool,
) -> Result<(), RunError>{
    let Ok(urls) = get_direct_urls(query, website) else {
        return Err(RunError::GetUrlError);
    };

    let mut i = 0;
    for url in urls {
        if i >= n_considered {
            break;
        }

        let Ok(bibtexes) = get_bibtex(&url, website) else {
            return Err(RunError::GetBibtexError{ url });
        };
        if bibtexes.len() != 1 {
            return Err(RunError::MultipleBibtexError{ url });
        }

        let bibtex = &bibtexes[0];
        if ignore_preprint && bibtex.replace(" ", "").contains("eprinttype={") {
            continue;
        }
        let bibtex = process_bibtex(bibtex, full_bibtex);

        println!("Trial {} - Get BibTeX:\n{}", &i, bibtex);

        if i < n_considered {
            println!("Satisfied? [ 'y' or 'Y' -> yes / others -> no ] ");
            stdout().flush().unwrap();
            let mut resp: String = String::new();
            stdin().read_line(&mut resp).unwrap();

            if resp.trim().to_lowercase() != "y" {
                i += 1;
                continue;
            } else {
                if let Err(_) = copy_text(&bibtex) {
                    return Err(RunError::CopyToClipboardError);
                };
                println!("BibTeX copied to your clipboard!");
                return Ok(());
            }
        }
    }

    println!("No satisfying BibTeX is found. Sorry...🤧");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_urls_test() {
        let query = "Attention is All you Need, vaswani";

        let res_dblp = get_direct_urls(query, Website::Dblp).unwrap();
        assert_eq!(
            res_dblp[0],
            "https://dblp.org/rec/conf/nips/VaswaniSPUJGKP17.html?view=bibtex"
        );
        assert_eq!(
            res_dblp[1],
            "https://dblp.org/rec/journals/corr/VaswaniSPUJGKP17.html?view=bibtex"
        );
    }

    #[test]
    fn get_bibtex_test() {
        let url = "https://dblp.org/rec/conf/nips/VaswaniSPUJGKP17.html?view=bibtex";
        assert!(get_bibtex(url, Website::Dblp).unwrap()[0]
            .starts_with("@inproceedings{DBLP:conf/nips/VaswaniSPUJGKP17,"))
    }
}
