#[macro_use]
extern crate clap;

use clap::{App, Arg};

use book_check::check_urls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("book-check").version(crate_version!()).arg(
        Arg::with_name("url-file")
            .takes_value(true)
            .required(true)
            .help("The file with a list of URLs to check"),
    );

    let args = app.get_matches();
    let url_file = args.value_of("url-file").unwrap();
    check_urls(url_file).await?;

    Ok(())
}
