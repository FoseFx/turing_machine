use std::fmt::Debug;
use std::rc::Rc;
use super::util::BLANK;
use super::util::TMDirection;
use super::tm_cell::{TMCellRef, TMCell};

pub struct TMTape {
  first: TMCellRef,
  last: TMCellRef,
  head: TMCellRef
}
impl TMTape {
  pub fn from(word: &str) -> Self {
    let mut first: Option<TMCellRef> = None;
    let mut last: Option<TMCellRef> = None;

    for c in word.chars() {
      let cell = TMCell::from(c);

      if first.is_none() {
        first = Some(cell.clone());
      }

      if last.is_some() {
        let last = last.unwrap();
        TMCell::append(last, cell.clone());
      }

      last = Some(cell);
    }

    if first.is_none() { // word is empty
      let cell = TMCell::blank();
      first = Some(cell.clone());
      last = Some(cell);
    }

    let first = first.unwrap();
    let last = last.unwrap();

    Self {
      first: first.clone(),
      last,
      head: first,
    }
  }

  pub fn set_value(&mut self, value: char) {
    self.head.borrow_mut().value = value;
  }

  pub fn read_value(&self) -> char {
    self.head.borrow().value
  }

  pub fn iter(&self) -> TMTapeForewardIterator {
    TMTapeForewardIterator {
      tape_head: self.head.clone(),
      iter_head: Some(self.first.clone())
    }
  }

  pub fn move_head(&mut self, direction: &TMDirection) {
    if *direction == TMDirection::N {
      return;
    }

    let head = self.head.borrow_mut();
    
    let mut other_cell = match direction {
      TMDirection::R => head.next.clone(),
      TMDirection::L => head.prev.clone(),
      TMDirection::N => unreachable!(),
    };
    drop(head);

    if other_cell.is_none() { // if other cell is none, put a blank there (and wire it up)
      let head_ref = self.head.clone();
      let blank = TMCell::blank();

      match direction {
        TMDirection::R => {
          TMCell::append(head_ref, blank.clone());
          self.last = blank.clone();
        },
        TMDirection::L => {
          TMCell::prepend(head_ref, blank.clone());
          self.first = blank.clone();
        }
        _ => unreachable!(),
      }

      other_cell = Some(blank);
    }

    let other_cell = other_cell.unwrap();
    self.head = other_cell;

    self.cleanup();
  }

  /**
   * move first/last pointers,
   * when they point to a Blank
   * and the head is not pointing there
   * 
   * this way we get rid of unused Blank Cells
   */
  fn cleanup(&mut self) {
    let head_is_last = Rc::ptr_eq(&self.head, &self.last);
    let head_is_first = Rc::ptr_eq(&self.head, &self.first);

    if self.first.borrow().value == BLANK && !head_is_first {

      let mut head = self.head.borrow_mut();
      head.prev = None;
      drop(head);

      self.first = self.head.clone();
    }

    if self.last.borrow().value == BLANK && !head_is_last {
      let mut head = self.head.borrow_mut();
      head.next = None;
      drop(head);

      self.last = self.head.clone();
    }
  }
}
impl Debug for TMTape {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    f.debug_list().entries(self.iter()).finish()
  }
}

pub struct TMTapeForewardIterator {
  tape_head: TMCellRef,
  iter_head: Option<TMCellRef>,
}
impl Iterator for TMTapeForewardIterator {
  type Item = (char, bool);
  fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
    if self.iter_head.is_none() {
      return None;
    }
    let head = self.iter_head.clone().unwrap();
    self.iter_head = head.borrow().next.clone();

    Some((head.clone().borrow().value, Rc::ptr_eq(&self.tape_head, &head)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from() {
    let tape = TMTape::from("abc");

    let a = tape.first;
    let b = a.borrow().next.clone().unwrap();
    let c = tape.last;

    // check values
    assert_eq!(a.borrow().value, 'a');
    assert_eq!(b.borrow().value, 'b');
    assert_eq!(c.borrow().value, 'c');

    // check next pointers
    assert!(Rc::ptr_eq(&a.borrow().next.clone().unwrap(), &b));
    assert!(Rc::ptr_eq(&b.borrow().next.clone().unwrap(), &c));
    assert!(c.borrow().next.clone().is_none());

    // check prev pointers
    assert!(a.borrow().prev.clone().is_none());
    assert!(Rc::ptr_eq(&b.borrow().prev.clone().unwrap(), &a));
    assert!(Rc::ptr_eq(&c.borrow().prev.clone().unwrap(), &b));

    // check head
    assert!(Rc::ptr_eq(&a, &tape.head));
  }

  #[cfg(test)]
  mod cleanup {
    use super::*;

    #[test]
    fn no_blanks() {
      let mut tape = TMTape::from("a");

      // check default head position
      assert_eq!(tape.head.clone().borrow().value, 'a');

      //
      // cleanup should do nothing, as first and last dont point to blanks
      //

      let old_first = tape.first.clone();
      let old_last = tape.last.clone();
      tape.cleanup();
      assert!(Rc::ptr_eq(&tape.first, &old_first));
      assert!(Rc::ptr_eq(&tape.last, &old_last));
      assert!(&tape.head.borrow().prev.clone().is_none());
      assert!(&tape.head.borrow().next.clone().is_none());
    }

    #[test]
    fn first_not_head() {
      //
      // cleanup should remove blank on first, when not head
      //

      let mut tape = TMTape::from("a");

      // fake left movement to create blank
      let blank = TMCell::blank();
      TMCell::prepend(tape.head.clone(), blank.clone());
      tape.first = blank;

      let old_first = tape.first.clone();

      tape.cleanup();
      assert!(!Rc::ptr_eq(&tape.first, &old_first));
      assert!(Rc::ptr_eq(&tape.first, &tape.head.clone()));
      assert!(&tape.head.clone().borrow().prev.is_none());
    }

    #[test]
    fn last_not_head() {
      //
      // cleanup should remove blank on last, when not head
      //

      let mut tape = TMTape::from("a");

      // fake right movement to create blank
      let blank = TMCell::blank();
      TMCell::append(tape.head.clone(), blank.clone());
      tape.last = blank;

      let old_last = tape.last.clone();

      tape.cleanup();
      assert!(!Rc::ptr_eq(&tape.last, &old_last));
      assert!(Rc::ptr_eq(&tape.last, &tape.head.clone()));
      assert!(&tape.head.clone().borrow().next.is_none());
    }

    #[test]
    fn first_head() {
      //
      // cleanup should not remove blank on first, when head
      //

      let mut tape = TMTape::from("a");

      // fake left movement to create blank
      let blank = TMCell::blank();
      TMCell::prepend(tape.head.clone(), blank.clone());
      tape.first = blank.clone();
      tape.head = blank;

      let old_first = tape.first.clone();

      tape.cleanup();
      assert!(Rc::ptr_eq(&tape.first, &old_first));
      assert!(Rc::ptr_eq(&tape.first, &tape.head.clone()));
    }

    #[test]
    fn last_head() {
      //
      // cleanup should not remove blank on last, when head
      //

      let mut tape = TMTape::from("a");

      // fake right movement to create blank
      let blank = TMCell::blank();
      TMCell::append(tape.head.clone(), blank.clone());
      tape.last = blank.clone();
      tape.head = blank;

      let old_last = tape.last.clone();

      tape.cleanup();
      assert!(Rc::ptr_eq(&tape.last, &old_last));
      assert!(Rc::ptr_eq(&tape.last, &tape.head.clone()));
    }
  }

  #[cfg(test)]
  mod move_head {
    use super::*;

    #[test]
    fn n() {
      // should not move when Direction::N

      let mut tape = TMTape::from("abc");

      let old_head = tape.head.clone();
      let old_first = tape.first.clone();
      let old_last = tape.last.clone();

      tape.move_head(&TMDirection::N);

      assert!(Rc::ptr_eq(&tape.head, &old_head));
      assert!(Rc::ptr_eq(&tape.first, &old_first));
      assert!(Rc::ptr_eq(&tape.last, &old_last));
    }

    #[test]
    fn r_with_next() {
      // should move to next when Direction::R

      let mut tape = TMTape::from("abc");

      let old_next = tape.head.borrow().next.clone().unwrap();
      let old_first = tape.first.clone();
      let old_last = tape.last.clone();

      tape.move_head(&TMDirection::R);

      assert!(Rc::ptr_eq(&tape.head, &old_next));
      assert!(Rc::ptr_eq(&tape.first, &old_first));
      assert!(Rc::ptr_eq(&tape.last, &old_last));
    }
  }

  #[test]
  fn r_without_next() {
    // should create Blank and move there

    let mut tape = TMTape::from("a");

    let old_head = tape.head.clone();
    let old_first = tape.first.clone();
    let old_last = tape.last.clone();

    tape.move_head(&TMDirection::R);

    assert_eq!(tape.head.clone().borrow().value, BLANK);
    assert!(&old_head.borrow().next.is_some());
    assert!(Rc::ptr_eq(&old_head.borrow().next.clone().unwrap().borrow().prev.clone().unwrap(), &old_head));
    assert!(!Rc::ptr_eq(&tape.head, &old_head));
    assert!(Rc::ptr_eq(&tape.first, &old_first));
    assert!(Rc::ptr_eq(&tape.last, &tape.head));
    assert!(!Rc::ptr_eq(&tape.last, &old_last));
  }

  
  #[test]
  fn l_with_prev() {
    // should move to next when Direction::R

    let mut tape = TMTape::from("abc");
    tape.move_head(&TMDirection::R);


    let old_prev = tape.head.borrow().prev.clone().unwrap();
    let old_first = tape.first.clone();
    let old_last = tape.last.clone();

    tape.move_head(&TMDirection::L);

    assert!(Rc::ptr_eq(&tape.head, &old_prev));
    assert!(Rc::ptr_eq(&tape.first, &old_first));
    assert!(Rc::ptr_eq(&tape.last, &old_last));
  }


  #[test]
  fn l_without_prev() {
    // should create Blank and move there

    let mut tape = TMTape::from("a");

    let old_head = tape.head.clone();
    let old_first = tape.first.clone();
    let old_last = tape.last.clone();

    tape.move_head(&TMDirection::L);

    assert_eq!(tape.head.clone().borrow().value, BLANK);
    assert!(&old_head.borrow().prev.is_some());
    assert!(Rc::ptr_eq(&old_head.borrow().prev.clone().unwrap().borrow().next.clone().unwrap(), &old_head));
    assert!(!Rc::ptr_eq(&tape.head, &old_head));
    assert!(!Rc::ptr_eq(&tape.first, &old_first));
    assert!(Rc::ptr_eq(&tape.first, &tape.head));
    assert!(Rc::ptr_eq(&tape.last, &old_last));
  }
}
