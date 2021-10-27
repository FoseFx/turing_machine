use std::fmt::Debug;
use crate::util::BLANK;
use std::rc::Rc;
use std::cell::RefCell;

pub type TMCellRef = Rc<RefCell<TMCell>>;

pub struct TMCell {
  pub value: char,
  pub prev: Option<TMCellRef>,
  pub next: Option<TMCellRef>,
}
impl TMCell {
  pub fn from(value: char) -> TMCellRef {
    Rc::from(
      RefCell::from(
        Self {
          value,
          prev: None,
          next: None,
        }
      )
    )
  }
  pub fn blank() -> TMCellRef {
    Self::from(BLANK)
  }

  // This is what we want in the end:
  //     b              a
  //       next ----->
  //            <-----   prev
  pub fn prepend(a_ref: TMCellRef, b_ref: TMCellRef) {
    let mut a = a_ref.borrow_mut();
    a.prev = Some(b_ref.clone());
    drop(a);

    let mut b = b_ref.borrow_mut();
    b.next = Some(a_ref);
  }

  
  // This is what we want in the end:
  //     a              b
  //       next ----->
  //            <-----   prev
  pub fn append(a_ref: TMCellRef, b_ref: TMCellRef) {
    let mut a = a_ref.borrow_mut();
    a.next = Some(b_ref.clone());
    drop(a);

    let mut b = b_ref.borrow_mut();
    b.prev = Some(a_ref);
  }  
}
impl Debug for TMCell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    fn fmt_ref(r: Option<TMCellRef>) -> String {
      if r.is_none() {
        return "None".to_string();
      }

      let r = r.unwrap();
      return format!("-> {}", r.borrow().value);
    }
    f.debug_struct("TMCell")
    .field("value", &self.value)
    .field("prev", &fmt_ref(self.prev.clone()))
    .field("next", &fmt_ref(self.next.clone()))
    .finish()
  }
}

#[cfg(test)]
mod tm_cell_test {
  use super::*;

  #[test]
  fn test_prepend() {
    let a = TMCell::from('a');
    let b = TMCell::from('b');
    
    // fresh cells point to nothing else yet
    assert!(a.borrow().prev.is_none());
    assert!(a.borrow().next.is_none());

    // prepend
    TMCell::prepend(a.clone(), b.clone());

    // check if b <-> a
    assert!(Rc::ptr_eq(&a.borrow().prev.clone().unwrap(), &b));
    assert!(Rc::ptr_eq(&b.borrow().next.clone().unwrap(), &a));
  }

  #[test]
  fn test_append() {
    let a = TMCell::from('a');
    let b = TMCell::from('b');
    
    // fresh cells point to nothing else yet
    assert!(a.borrow().prev.is_none());
    assert!(a.borrow().next.is_none());

    // append
    TMCell::append(a.clone(), b.clone());

    // check if a <-> b
    assert!(Rc::ptr_eq(&a.borrow().next.clone().unwrap(), &b));
    assert!(Rc::ptr_eq(&b.borrow().prev.clone().unwrap(), &a));
  }

}
