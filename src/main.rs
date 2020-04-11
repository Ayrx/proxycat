use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("PACKAGE")
                .help("Android app to proxy.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PROXY")
                .help("Proxy address to use.")
                .required(true)
                .index(2),
        )
        .get_matches();

    let package_name = matches.value_of("PACKAGE").unwrap();
    let proxy = matches.value_of("PROXY").unwrap();
}
