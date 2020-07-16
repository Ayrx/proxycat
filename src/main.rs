use anyhow::{bail, Result};
use clap::{App, AppSettings, Arg, SubCommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let matches = App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("add")
                .about("Add proxy rule.")
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
                ),
        )
        .subcommand(SubCommand::with_name("clean").about("Remove iptable NAT rules."))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        add(matches)?;
    }

    if matches.subcommand_matches("clean").is_some() {
        clean()?;
    }

    Ok(())
}

fn add(matches: &clap::ArgMatches) -> Result<()> {
    let package_name = matches.value_of("PACKAGE").unwrap();
    let proxy = matches.value_of("PROXY").unwrap();

    let packages = parse_packages_list()?;

    if let Some(v) = packages.get(package_name) {
        setup_proxycat_chain()?;
        insert_iptable_rule(v, proxy)?;
    } else {
        bail!("Package {} not installed on device.", package_name);
    }

    Ok(())
}

fn clean() -> Result<()> {
    let status = Command::new("iptables")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&["-t", "nat", "-n", "-L", "PROXYCAT"])
        .status()?;

    if status.success() {
        Command::new("iptables")
            .args(&["-t", "nat", "-F", "PROXYCAT"])
            .status()?;

        Command::new("iptables")
            .args(&["-t", "nat", "-D", "OUTPUT", "-j", "PROXYCAT"])
            .status()?;

        Command::new("iptables")
            .args(&["-t", "nat", "-X", "PROXYCAT"])
            .status()?;
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

fn setup_proxycat_chain() -> Result<()> {
    let status = Command::new("iptables")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(&["-t", "nat", "-n", "-L", "PROXYCAT"])
        .status()?;

    if !status.success() {
        Command::new("iptables")
            .args(&["-t", "nat", "-N", "PROXYCAT"])
            .status()?;

        Command::new("iptables")
            .args(&["-t", "nat", "-I", "OUTPUT", "-j", "PROXYCAT"])
            .status()?;
    }

    Ok(())
}

fn insert_iptable_rule(uid: &str, proxy: &str) -> Result<()> {
    Command::new("iptables")
        .args(&[
            "-t",
            "nat",
            "-A",
            "PROXYCAT",
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
