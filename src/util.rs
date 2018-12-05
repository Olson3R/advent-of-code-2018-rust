use std;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct Util;

type Result<T> = std::result::Result<T, Box<Error>>;

impl Util {
  pub fn load_input(file: &str) -> Result<Vec<String>> {
    let f = File::open(file)?;
    let mut br = BufReader::new(&f);
    let lines = br.lines()
      .map(|line| Ok(line?.parse()?) )
      .collect::<Result<_>>()?;
    Ok(lines)
  }
}
