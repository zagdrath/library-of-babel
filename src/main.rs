// Copyright (C) 2022, Zagdrath. This is open source software, you can
// modify and / or share it under the terms of the GNU GPLv3 license file in the
// root directory of this project.

// External crate dependencies
extern crate babel;
extern crate clap;

use babel::{read, search, Address};
use clap::{App, Arg};
use std::process;

macro_rules! parse_address {
    ($input:expr, $max:expr, $label:expr) => {
        match $input.parse::<u32>() {
            Ok(n) => {
                if n >= $max {
                    println!("Bad address: {} must be less than {}", $label, $max);
                    process::exit(1);
                }
                n
            }
            Err(_) => {
                println!("Bad address: Not a number.");
                process::exit(1);
            }
        }
    };
}

fn main() {
    let matches = App::new("The Library of Babel")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Home of every book ever")
        .author("Zagdrath <zagdrath@member.fsf.org>")
        .subcommand(
            App::new("search")
                .about("Search the library for something")
                .version(env!("CARGO_PKG_VERSION"))
                .arg(
                    Arg::new("query")
                        .required(true)
                        .help("The search query")
                        .use_delimiter(false),
                )
                .arg(
                    Arg::new("noisy")
                        .long("noisy")
                        .help("Allows pages with random characters around the query"),
                ),
        )
        .subcommand(
            App::new("read")
                .about("Read a page from the library")
                .version(env!("CARGO_PKG_VERSION"))
                .arg(Arg::new("address").required(true).help(
                    "The address of the page to read from (wall:shelf:volume:page:hex_address)",
                )),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("search") {
        let query = matches.value_of("query").unwrap();

        let query = if matches.is_present("noisy") {
            babel::pad_rand(query)
        } else {
            query.to_owned()
        };

        let addr = search(&query);

        println!("Page found!\n\"{}\"", addr);
    } else if let Some(matches) = matches.subcommand_matches("read") {
        let address = matches.value_of("address").unwrap();

        let split: Vec<&str> = address.split(':').collect();

        if split.len() != 5 {
            println!("Bad address: (wall:shelf:volume:page:hex_address)");
            process::exit(1);
        }

        let addr = Address {
            hex: split[4].to_owned(),
            wall: parse_address!(split[0], 4, "Wall"),
            shelf: parse_address!(split[1], 5, "Shelf"),
            volume: parse_address!(split[2], 32, "Volume"),
            page: parse_address!(split[3], 410, "Page"),
        };

        let page = read(&addr);
        print_formatted(&page);
    }
}

/// Prints the page with correct formatting
fn print_formatted(page: &str) {
    for (i, a) in page.chars().enumerate() {
        print!("{}", a);
        if (i + 1) % 80 == 0 {
            println!("");
        }
    }
}
