use anyhow::Result;
use clap::Parser;

mod args;
mod save_helper;
mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
    let args = args::Args::parse();

    let html = scraper::get_html(&args.url).await?;
    let links = scraper::extract_chapter_links(&html);

    let end = args.end.unwrap_or(links.len() - 1);
    let links_to_extract = links.get(args.start..=end).unwrap().to_vec();

    let chapters = scraper::extract_chapters(&links_to_extract).await?;
    let combined_content = chapters.join("\n\n\n");

    save_helper::remove_path(&args.out)?;
    save_helper::save_file(&combined_content, &args.out)?;

    return Ok(());
}
