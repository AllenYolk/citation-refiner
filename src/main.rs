use clap::{Parser, ValueEnum};

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
    n_considered: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Website {
    Dblp,
}

fn main() {
    let args = Cli::parse();
    dbg!(args);
}
