use anyhow::{bail, Result};
use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() -> Result<()> {
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

    let packages = parse_packages_list()?;

    if let Some(v) = packages.get(package_name) {
        insert_iptable_rule(v, proxy)?;
    } else {
        bail!("Package {} not installed on device.", package_name);
    }

    Ok(())
}

fn parse_packages_list() -> Result<HashMap<String, String>> {
    let file = File::open("/data/system/packages.list")?;

    let mut map = HashMap::new();

    for line in BufReader::new(file).lines() {
        let line = line?;
        let mut l = line.split_ascii_whitespace();

        let package_name = l.next().unwrap().to_string();
        let package_uid = l.next().unwrap().to_string();

        map.insert(package_name, package_uid);
    }

    Ok(map)
}

fn insert_iptable_rule(uid: &str, proxy: &str) -> Result<()> {
    Command::new("iptables")
        .args(&[
            "-t",
            "nat",
            "-A",
            "OUTPUT",
            "-m",
            "owner",
            "--uid-owner",
            uid,
            "-p",
            "tcp",
            "-j",
            "DNAT",
            "--to-destination",
            proxy,
        ])
        .status()?;

    Ok(())
}
