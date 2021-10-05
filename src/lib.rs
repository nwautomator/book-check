extern crate term;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use url::Url;

type Urls = Vec<String>;
type CheckedUrls = HashMap<String, u16>;

enum Status {
    Success(String),
    Error(String),
    Warning(String),
}

impl Status {
    fn new(status_code: u16, msg: &str) -> Self {
        match status_code {
            200..=299 => {
                let message = format!("{} - {}", status_code, msg);
                Status::Success(message)
            }
            400..=499 => {
                let message = format!("{} - {}", status_code, msg);
                Status::Warning(message)
            }
            _ => {
                let message = format!("{} - {}", status_code, msg);
                Status::Error(message)
            }
        }
    }
    fn print(&self) {
        let mut t = term::stdout().unwrap();
        match self {
            Status::Success(msg) => {
                t.fg(term::color::GREEN).unwrap();
                write!(t, "◉ ").unwrap();
                t.fg(term::color::WHITE).unwrap();
                writeln!(t, "{}", msg).unwrap();
            }
            Status::Error(msg) => {
                t.fg(term::color::RED).unwrap();
                write!(t, "◉ ").unwrap();
                t.fg(term::color::WHITE).unwrap();
                writeln!(t, "{}", msg).unwrap();
            }
            Status::Warning(msg) => {
                t.fg(term::color::YELLOW).unwrap();
                write!(t, "◉ ").unwrap();
                t.fg(term::color::WHITE).unwrap();
                writeln!(t, "{}", msg).unwrap();
            }
        }
        t.reset().unwrap();
    }
}

async fn _make_urls(fname: &str) -> Result<Urls, Box<dyn std::error::Error>> {
    let mut urls: Urls = Vec::new();

    let f = File::open(fname).with_context(|| format!("Couldn't open '{}'", fname))?;
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap().to_lowercase();

        // skip blank lines
        if line.is_empty() {
            continue;
        }

        // skip lines without a URL
        if line.chars().nth(0).unwrap() != 'h' {
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
            Err(_) => 522 as u16, // Use CloudFlare's error status code
        };
        result.insert(link.to_string(), status);
    }
    Ok(result)
}

pub async fn check_urls(url_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let urls = _make_urls(url_file).await?;
    let results = _check_urls(urls).await?;

    for (link, status) in results {
        let res = Status::new(status, &link);
        res.print();
    }

    Ok(())
}
