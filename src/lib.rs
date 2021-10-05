extern crate term;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use url::{ParseError, Url};

type Urls = Vec<String>;
type CheckedUrls = HashMap<String, u16>;

async fn _make_urls(fname: &str) -> Result<Urls, ParseError> {
    let mut urls: Urls = Vec::new();

    let f = File::open(fname).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap().to_lowercase();
        if line.is_empty() {
            continue;
        }
        if line.chars().nth(0).unwrap() == '#' {
            continue;
        }
        let link = Url::parse(&line)?;
        match link.scheme() {
            "http" | "https" => urls.push(link.to_string()),
            _ => eprintln!("Couldn't parse URL '{}'", link),
        }
    }
    Ok(urls)
}

async fn _check_urls(urls: Vec<String>) -> Result<CheckedUrls, Box<dyn std::error::Error>> {
    println!("Checking {} URLs...", urls.len());
    let mut result: CheckedUrls = HashMap::new();
    for link in &urls {
        let resp = reqwest::get(link).await;
        let status = match resp {
            Ok(res) => res.status().as_u16(),
            Err(_) => 999 as u16,
        };
        result.insert(link.to_string(), status);
    }
    Ok(result)
}

pub async fn check_urls(url_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let urls = _make_urls(url_file).await?;
    let results = _check_urls(urls).await?;

    let mut t = term::stdout().unwrap();
    for (link, status) in results {
        match status {
            200..=299 => {
                t.fg(term::color::GREEN).unwrap();
                write!(t, "◉ ").unwrap();
                t.fg(term::color::WHITE).unwrap();
                writeln!(t, "{}", link).unwrap();
            }
            400..=599 => {
                t.fg(term::color::YELLOW).unwrap();
                write!(t, "◉ ").unwrap();
                t.fg(term::color::WHITE).unwrap();
                writeln!(t, "{}", link).unwrap();
            }
            999 => {
                t.fg(term::color::RED).unwrap();
                write!(t, "◉ ").unwrap();
                t.fg(term::color::WHITE).unwrap();
                writeln!(t, "{} - Error: couldn't connect", link).unwrap();
            }
            _ => {
                t.fg(term::color::RED).unwrap();
                write!(t, "◉ ").unwrap();
                t.fg(term::color::WHITE).unwrap();
                writeln!(t, "{}", link).unwrap();
            }
        }
    }

    t.reset().unwrap();

    Ok(())
}
