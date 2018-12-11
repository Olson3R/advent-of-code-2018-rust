#[macro_use]
extern crate advent_of_code_2018_rust;

use advent_of_code_2018_rust::util::Util;

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
                for b in (b'a'..=b'z') {
                    let r1 = (b as char).to_string().to_uppercase() + &(b as char).to_string();
                    let r2 = r1.clone().chars().rev().collect::<String>();
                    polymer = polymer.replace(&r1, "").replace(&r2, "");
                }
            }
            polymer.len()
        })
        .min()
        .expect("Something");

    println!("Min {:?}", min);
}
