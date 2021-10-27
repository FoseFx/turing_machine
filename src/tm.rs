use super::util::BLANK;
use super::util::TMDeltaTable;
use super::tm_tape::TMTape;

pub struct TM {
  state: usize,
  end_state: usize,
  delta: TMDeltaTable,
}
impl TM {
  pub fn new(start_state: usize, end_state: usize, delta: TMDeltaTable) -> Self {
    Self {
      state: start_state,
      end_state: end_state,
      delta,
    }
  }

  pub fn run(&mut self, word: &str) {
    let mut tape = TMTape::from(word);

    while self.state != self.end_state {
      self.print_config(&tape);

      let key = (self.state, tape.read_value());
      let delta_entry = self.delta.get(&key);
      if delta_entry.is_none() {
        panic!("The TM reached a configuration that it was not programmed for");
      }
      let (next_state, next_value, direction) = delta_entry.unwrap();
      self.state = *next_state;
      tape.set_value(*next_value);
      tape.move_head(direction);
    }

    self.print_config(&tape);
  }

  fn print_config(&self, tape: &TMTape) {

    print!("...");
    print!("{}", BLANK);
    for (value, is_head) in tape.iter() {
      if is_head {
        print!("[{}]", self.state);
      }
      print!("{}", value);
    }
    print!("{}", BLANK);
    println!("...");
  }
}
