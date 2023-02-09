use std::{collections::HashSet, hash::Hash};

pub fn run() {

}

trait USet<T> {
  fn size(&self) -> usize;
  fn add(&mut self, item: T) -> bool;
  fn remove(&mut self, item: &T) -> Option<T>;
  fn find(&self, item: &T) -> Option<&T>;
}

struct Bag<T> where T: PartialEq + Eq + Hash {
  inner: HashSet<(T, HashSet<T>)>
}

impl<T> Bag<T> where T: PartialEq + Eq + Hash {
  pub fn new() -> Self {
    Bag { inner: HashSet::new() }
  }

  pub fn size(&self) -> usize {
    todo!()
  }

  pub fn add(&mut self, item: T) -> bool {
    todo!()
  }

  pub fn remove(&mut self, item: &T) -> Option<T> {
    todo!()
  }

  pub fn find(&self, item: &T) -> Option<&T> {
    todo!()
  }

  pub fn find_all(&self, item: &T) -> Option<Vec<&T>> {
    todo!()
  }
}
