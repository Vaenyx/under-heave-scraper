use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Scrape under heaven novels")]
pub struct Args {
    #[arg(short, long, help = "Url to any chapter of the novel")]
    pub url: String,

    #[arg(
        short,
        long,
        help = "Output file",
        default_value_t = String::from("out")
    )]
    pub out: String,

    #[arg(
        short,
        long,
        help = "First chapter to extract (chapters start at 0)",
        default_value_t = 0
    )]
    pub start: usize,

    #[arg(short, long, help = "Last chapter to extract (chapters start at 0)")]
    pub end: Option<usize>,
}
