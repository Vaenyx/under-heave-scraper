use anyhow::{Ok, Result};
use reqwest;
use scraper::{Html, Selector};
use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub struct Chapter {
    pub title: String,
    pub content: String,
}

pub async fn get_html(url: &str) -> Result<Html> {
    let html = reqwest::get(url).await?.text().await?;
    sleep(Duration::from_secs(1)).await;
    return Ok(Html::parse_document(&html));
}

pub fn extract_chapter_links(html: &Html) -> Vec<String> {
    let selector = Selector::parse("#chapter-index-list li a").unwrap();

    return html
        .select(&selector)
        .filter_map(|a| a.value().attr("href"))
        .map(String::from)
        .collect();
}

fn extract_chapter(html: &Html) -> Chapter {
    let title_selector = Selector::parse(".chapter__title").unwrap();

    let title = html
        .select(&title_selector)
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or_default();

    let content_selector = Selector::parse("#chapter-content p").unwrap();

    let content = html
        .select(&content_selector)
        .map(|p| p.text().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n\n");

    return Chapter { title, content };
}

pub async fn extract_chapters(links: &Vec<String>) -> Result<Vec<String>> {
    println!("Started extraction of {} chapters", links.len());

    let mut res = vec![];
    for (idx, link) in links.iter().enumerate() {
        let html = get_html(&link).await?;
        let chapter = extract_chapter(&html);

        let formatted_chapter = format!("{}\n\n\n{}", chapter.title, chapter.content);
        res.push(formatted_chapter);

        let percent = (idx + 1) as f64 / links.len() as f64 * 100.0;

        io::stdout().flush().unwrap();
        print!(
            "\rExtraction: {}/{} ({:.1}%)",
            idx + 1,
            links.len(),
            percent
        );
    }

    println!("\nFinished exatraction");
    return Ok(res);
}
