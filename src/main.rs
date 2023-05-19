use citation_refiner::{run, Website, RunError};
use clap::Parser;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    query: Option<String>,

    #[arg(short, long)]
    #[arg(value_enum)]
    #[arg(default_value_t = Website::Dblp)]
    website: Website,

    #[arg(short, long)]
    #[arg(default_value_t = 5)]
    n_considered: usize,

    #[arg(short, long)]
    ignore_preprint: bool,

    #[arg(short, long)]
    full_bibtex: bool,
}

fn main() {
    let args = Cli::parse();
    let query = args.query.as_deref().expect("No query provided");
    let website = args.website;
    let n_considered = args.n_considered;
    let ignore_preprint = args.ignore_preprint;
    let full_bibtex = args.full_bibtex;

    match run(query, website, n_considered, ignore_preprint, full_bibtex) {
        Err(e) => {
            match e {
                RunError::GetUrlError => {
                    eprintln!("Error: cannot get direct urls from website {:?}", website);
                },
                RunError::GetBibtexError{ url } => {
                    eprintln!("Error: cannot get bibtex from url {}", url);
                },
                RunError::MultipleBibtexError{url} => {
                    eprintln!("Error: found multiple bibtex entries on {}, \nwhich is problematic!", url)
                },
                RunError::CopyToClipboardError => {
                    eprintln!("Error: cannot copy bibtex to the clipboard!");
                }
            }
            exit(-1);
        },
        _ => { () }
    }
}
