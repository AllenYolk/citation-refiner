use citation_refiner::{run, Website};
use clap::Parser;

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
}

fn main() {
    let args = Cli::parse();
    let query = args.query.as_deref().expect("No query provided");
    let website = args.website;
    let n_considered = args.n_considered;
    let ignore_preprint = args.ignore_preprint;

    run(query, website, n_considered, ignore_preprint)
}
