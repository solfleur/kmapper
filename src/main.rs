use std::fmt;
use inquire::{Text};
use clap::{Parser, ValueEnum};
use regex_lite::Regex;

#[derive(ValueEnum, Clone, Debug)]
enum MapperMode {
    ABCD,
    ABC,
    AB
}

impl fmt::Display for MapperMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MapperMode::ABCD => "abcd",
            MapperMode::ABC => "abc",
            MapperMode::AB => "ab"
        }) // or match on self and return custom strings
    }
}


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_missing_value = None)]
    inds: Option<String>,
    
    #[arg(short, long, default_value_t = MapperMode::ABCD)]
    mode: MapperMode
}

// Validate that input matches numlist pattern ([0-9]*,)*([0-9]*){1} (no spaces) and parse
fn parse_nlist(cand: &str) -> Result<Vec<u8>, ()> {
    let ns_cand: String = cand.split_whitespace().collect();
    let re = Regex::new("([0-9]*,)*([0-9]*){1}").unwrap();

    // TODO: implement selection for A/BC and A/B; currently only AB/CD and doesn't account for OOB.
    if re.is_match(&ns_cand) {
        Ok(ns_cand.split(',').map(|i| i.parse::<u8>().unwrap()).collect())
    } else {
        Err(())
    }
}

// Vector to Comma-Separated List
fn vec2csl<T: ToString>(vec: &Vec<T>) -> String {
    // https://users.rust-lang.org/t/converting-a-vec-to-a-string-of-comma-separated-values/62853/4
    vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
}

// Get mapper LUT based on mappermode.
fn mode2mlut(mode: &MapperMode) -> [u8; 16] {
    let mut lut = [0u8; 16];

    match mode {
        MapperMode::ABCD => {
            // [0] = 0 already
            lut[4] = 1;
            lut[12] = 3;
            lut[8] = 2;
            lut[1] = 4;
            lut[5] = 5;
            lut[13] = 7;
            lut[9] = 6;
            lut[3] = 12;
            lut[7] = 13;
            lut[15] = 15;
            lut[11] = 14;
            lut[2] = 8;
            lut[6] = 9;
            lut[14] = 11;
            lut[10] = 10;
        }

        MapperMode::ABC => {
            // [0] = 0
            lut[4] = 1;
            lut[1] = 2;
            lut[5] = 3;
            lut[3] = 6;
            lut[7] = 7;
            lut[2] = 4;
            lut[6] = 5;
        }

        MapperMode::AB => {
            // [0] = 0;
            lut[2] = 1;
            lut[1] = 2;
            lut[3] = 3;
        }
    }

    lut
}

fn main() {
    let args = Cli::parse();

    let strind = args.inds;

    let inds = {
        if strind.is_some() {
            parse_nlist(&strind.unwrap()).unwrap()
        } else {
            let mut result = Err(());

            while result.is_err() {
                result = parse_nlist(&*Text::new("Enter K indices.").prompt().unwrap());
            };

            result.unwrap()
        }
    };


    let minds = vec2csl(&inds.iter().map(|i| mode2mlut(&args.mode)[*i as usize]).collect::<Vec<u8>>());
    // TODO: visual kmap
    println!("\nMaps to {}.", &minds);
}
