mod util;
mod tm;
mod tm_cell;
mod tm_tape;

use std::collections::HashMap;
use crate::util::TMDirection;
use crate::tm::TM;
use crate::util::TMDeltaTable;


fn main() {
    //
    // set up TM
    //

    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1);
    let word = args.get(2);

    if filename.is_none() || word.is_none() {
        eprintln!("use: tm <file.tm> <word>");
        std::process::exit(1);
    }

    let tm_string = std::fs::read_to_string(filename.unwrap())
        .expect("Could not read file, make sure the file exists and it's permissions are set correctly");

    let mut tm = parse_tm(tm_string);

    //
    // run TM
    //

    let word = word.unwrap();
    tm.run(word);
}


//
// Parser and Helper functions
//

fn parse_as_usize(line: &str) -> usize {
  return line
      .parse::<usize>()
      .unwrap(); // keine Zahl = crash
}

fn parse_delta_entry(line: &str) -> ((usize, char), (usize, char, TMDirection)) {
  let mut tokens = line.split_whitespace();
  let current_q = parse_as_usize(tokens.next().unwrap());
  let current_val = tokens.next().unwrap().parse::<char>().unwrap();
  let next_q = parse_as_usize(tokens.next().unwrap());
  let next_val = tokens.next().unwrap().parse::<char>().unwrap();
  let direction = TMDirection::from(tokens.next().unwrap());

  let key = (current_q, current_val);
  let value = (next_q, next_val, direction);

  return (key, value);
}

fn parse_tm(tm_string: String) -> TM {
  let mut lines = tm_string.lines();

  let _n: usize = parse_as_usize(lines.next().unwrap());
  let _sigma = lines.next().unwrap();
  let _gamma = lines.next().unwrap();
  let i: usize = parse_as_usize(lines.next().unwrap());
  let j: usize = parse_as_usize(lines.next().unwrap());

  let mut delta: TMDeltaTable = HashMap::new();

  for line in lines {
      let (key, value) = parse_delta_entry(line);
      delta.insert(key, value);
  }

  TM::new(i, j, delta)
}
