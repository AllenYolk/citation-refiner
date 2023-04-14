use clap::Parser;
use citation_refiner::{Website, run};

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
    #[arg(default_value_t = 1)]
    n_considered: usize,
}

fn main() {
    let args = Cli::parse();
    let query = args.query.as_deref().expect("No query provided");
    let website = args.website;
    let n_considered = args.n_considered;

    run(query, website, n_considered)
}
