/*
Copyright (C) 2021 Christopher Freas (code@packetbusters.net)

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of
the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 59 Temple Place, Suite 330,
Boston, MA 02111-1307 USA
*/

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
