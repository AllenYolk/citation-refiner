const FILTERED_FIELDS: [&str; 4] = ["url", "timestamp", "biburl", "bibsource"];

pub fn process_bibtex(bibtex: &str, full_bibtex: bool) -> String {
    if full_bibtex {
        return bibtex.to_string();
    }

    let mut ans = String::new();
    let bibtex= bibtex
        .split('\n')
        .filter(|s| !FILTERED_FIELDS.iter().any(|ss| s.trim().starts_with(*ss)));

    for l in bibtex {
        ans.push_str(l);
        ans.push('\n');
    }

    ans[..ans.len() - 1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_bibtex_test() {
        let bibtex = r#"@inproceedings{x,
            author       = {Ashish Vaswani},
            editor       = {Isabelle Guyon},
            title        = {Attention is All you Need},
            booktitle    = {Advances in Neural Information Processing Systems},
            url          = {https://proceedings.neurips.cc/paper/2017/hash/3f5ee243547dee91fbd053c1c4a845aa-Abstract.html},
            pages        = {5998--6008},
            year         = {2017},
            timestamp    = {Thu, 21 Jan 2021 15:15:21 +0100},
            biburl       = {https://dblp.org/rec/conf/nips/VaswaniSPUJGKP17.bib},
            bibsource    = {dblp computer science bibliography, https://dblp.org}
          }"#;

        let res = process_bibtex(bibtex, false);

        let correct = r#"@inproceedings{x,
            author       = {Ashish Vaswani},
            editor       = {Isabelle Guyon},
            title        = {Attention is All you Need},
            booktitle    = {Advances in Neural Information Processing Systems},
            pages        = {5998--6008},
            year         = {2017},
          }"#;

        assert_eq!(res, correct);
    }
}
