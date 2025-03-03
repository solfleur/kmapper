use inquire::{Text};
use clap::Parser;
use regex_lite::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_missing_value = None)]
    inds: Option<Vec<u8>>
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

fn main() {
    let args = Cli::parse();

    let inds = args.inds.unwrap_or_else(|| {
        let mut result = Err(());
        while result.is_err() {
            result = parse_nlist(&*Text::new("Enter K indices.").prompt().unwrap());
        }

        result.unwrap()
    });

    let mut lut = [0u8; 16];

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

    let minds = vec2csl(&inds.iter().map(|i| lut[*i as usize]).collect::<Vec<u8>>());
    // TODO: visual kmap
    println!("\nMaps to {}.", &minds);
}
