#[macro_use]
extern crate lazy_static;
extern crate advent_of_code_2018_rust;
extern crate regex;

use advent_of_code_2018_rust::util::Util;
use regex::Regex;

lazy_static! {
  static ref REGEX: Regex = Regex::new(r"aA|Aa|bB|Bb|cC|Cc|dD|Dd|eE|Ee|fF|Ff|gG|Gg|hH|Hh|iI|Ii|jJ|Jj|kK|Kk|lL|Ll|mM|Mm|nN|Nn|oO|Oo|pP|Pp|qQ|Qq|rR|Rr|sS|Ss|tT|Tt|uU|Uu|vV|Vv|wW|Ww|xX|Xx|yY|Yy|zZ|Zz").unwrap();
}

fn main() {
    let lines = Util::load_input("input/d5-input.txt").expect("Failed to read the input file.");

    let min: usize = (b'a'..=b'z')
        .map(|a| {
            let mut polymer = lines[0].clone();
            let mut prev = 0;

            let upper = (a as char).to_string().to_uppercase();
            let lower = (a as char).to_string();
            polymer = polymer.replace(&upper, "").replace(&lower, "");
            println!("{}", a);
            while prev != polymer.len() {
                prev = polymer.len();
                polymer = REGEX.replace_all(&polymer, "").to_string();
            }
            polymer.len()
        })
        .min()
        .expect("Something");

    println!("Min {:?}", min);
}
