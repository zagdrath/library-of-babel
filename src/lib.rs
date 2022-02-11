// Copyright (C) 2022, Zagdrath. This is open source software, you can
// modify and / or share it under the terms of the GNU GPLv3 license file in the
// root directory of this project.

// External crate dependencies
extern crate num;
extern crate rand;

use num::cast::{FromPrimitive, ToPrimitive};
use num::{BigInt, Integer, Signed, Zero};
use rand::Rng;

/// BABEL set of characters
pub const BABEL_SET: [char; 29] = [
    ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ',', '.',
];

/// BASE64 set of characters
pub const BASE64_SET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '-', '_',
];

/// Number of ROWS in a page
pub const ROWS: usize = 40;

/// Number of COLUMNS in a page
pub const COLUMNS: usize = 80;

/// Total PAGE_LENGTH
pub const PAGE_LENGTH: usize = ROWS * COLUMNS;

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

#[derive(Debug)]
/// Struct containting the address to a page in a volume on a shelf in the wall in a hex room
pub struct Address {
    /// Hex room
    pub hex: String,
    /// Wall
    pub wall: u32,
    /// Shelf
    pub shelf: u32,
    /// Volume
    pub volume: u32,
    /// Page
    pub page: u32,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.wall, self.shelf, self.volume, self.page, self.hex
        )
    }
}

/// Searching the library for a specific page
pub fn search(value: &str) -> Address {
    // Checking to make sure the input is the correct PAGE_LENGTH
    let mut value = format!("{1:<0$}", PAGE_LENGTH, value);
    value.truncate(PAGE_LENGTH);

    let mut rng = rand::thread_rng();

    // Randomly generates the location that this page will be located within the hex
    let wall = rng.gen_range(0..=4);
    let shelf = rng.gen_range(0..=5);
    let volume = rng.gen_range(0..=32);
    let page = rng.gen_range(0..=410);

    // Combines the location into a single unique number per hex
    let loc = wall * 1_000_000 + shelf * 100_000 + volume * 1_000 + page;
    let loc = BigInt::from_u32(loc).unwrap();

    // Creates a huge multiplier which when multiplited onto loc and it
    // simulates randomness but in a predictable and reversable way
    let mul = num::pow::pow(BigInt::from_u32(30).unwrap(), PAGE_LENGTH);

    // Finds the hex room address based on the desired
    // page contents and randomly decided upon location
    let hex_addr = to_arb_base(from_babel(value) + loc * mul, BASE64_SET.to_vec());

    Address {
        hex: hex_addr,
        wall: wall,
        shelf: shelf,
        volume: volume,
        page: page,
    }
}

/// Reads a page at a specific address in the library
pub fn read(addr: &Address) -> String {
    // Creates a location identifier and a huge multiplier in
    // the same exact way as was done in the search function
    let loc = addr.wall * 1_000_000 + addr.shelf * 100_000 + addr.volume * 1_000 + addr.page;
    let loc = BigInt::from_u32(loc).unwrap();

    let mul = num::pow::pow(BigInt::from_u32(30).unwrap(), PAGE_LENGTH);

    // Finds the page contents based on the hex room address and supplied location
    to_babel(from_arb_base(addr.hex.clone(), BASE64_SET.to_vec()) - loc * mul)
}

/// Convert from the BABEL character set to decimal BigInt
fn from_babel(value: String) -> BigInt {
    // TODO: Return result
    from_arb_base(value, BABEL_SET.to_vec())
}

/// Convert from decimal BigInt to the BABEL character set
fn to_babel(value: BigInt) -> String {
    // TODO: Return result
    to_arb_base(value, BABEL_SET.to_vec())
}

/// Convert from an arbitrary base with a character set to decimal BigInt
fn from_arb_base(value: String, set: Vec<char>) -> BigInt {
    // TODO: Return result
    let mut result = BigInt::zero();

    let base = BigInt::from_usize(set.len()).unwrap();

    for bn in value.chars() {
        let val = set.iter().position(|&b| bn == b).unwrap();
        let val = BigInt::from_usize(val).unwrap();

        result = &result * &base + &val;
    }

    result
}

/// Convert from decimal BigInt to some arbitrary base with a character set
fn to_arb_base(mut value: BigInt, set: Vec<char>) -> String {
    // TODO: Return result
    if value.is_negative() {
        value = -value;
    }

    let base = BigInt::from_usize(set.len()).unwrap();

    let mut arb = String::with_capacity(4096);

    loop {
        let (new_val, rem) = value.div_mod_floor(&base);

        arb.push(set[rem.to_usize().unwrap()]);

        value = new_val;

        if value.is_zero() {
            break;
        }
    }

    arb.chars().rev().collect()
}

/// Return a string randomly padded with BABEL characters
pub fn pad_rand(value: &str) -> String {
    if value.len() >= PAGE_LENGTH {
        return String::from(value);
    }
    let mut page = String::with_capacity(PAGE_LENGTH);

    let mut rng = rand::thread_rng();

    let before = rng.gen_range(0..PAGE_LENGTH - value.len());

    for _ in 0..before {
        page.push(*rng.choose(&BABEL_SET).unwrap());
    }

    page.push_str(value);

    while page.len() < page.capacity() {
        page.push(*rng.choose(&BABEL_SET).unwrap());
    }

    page
}
