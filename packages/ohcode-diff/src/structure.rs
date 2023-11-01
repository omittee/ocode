use std::rc::Rc;

pub struct Range {
    pub start_row: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub end_col: usize,
}
pub struct RowRange {
    pub start_row: usize,
    pub end_row: usize,
}
pub struct DiffDetail {
    pub origin: Range,
    pub modified: Range,
}
pub struct DiffRes {
    pub origin: RowRange,
    pub modified: RowRange,
}

impl Range {
    pub fn new(start_row: usize, end_row: usize, start_col: usize, end_col: usize) -> Self {
        Self {
            start_row,
            end_row,
            start_col,
            end_col,
        }
    }
    pub fn is_empty(&self) -> bool {
      self.start_row == self.end_row && self.start_col == self.end_col
    }
}

impl DiffDetail {
    pub fn new(origin: Range, modified: Range) -> Self {
        Self { origin, modified }
    }
}

impl RowRange {
    pub fn new(start_row: usize, end_row: usize) -> Self {
        Self { start_row, end_row }
    }
    pub fn is_empty(&self) -> bool {
      self.start_row == self.end_row
    }
    pub fn delta(&mut self, offset: usize) {
      self.start_row += offset;
      self.end_row += offset;
    }
}

impl DiffRes {
    pub fn new(origin: RowRange, modified: RowRange) -> Self {
        Self {
            origin,
            modified
        }
    }
    pub fn delta(&mut self, offset: usize) {
      self.origin.delta(offset);
      self.modified.delta(offset);
    }
}

#[derive(Debug)]
pub struct BiVec<T> {
    pos: Vec<T>,
    neg: Vec<T>,
}

impl<T: Default> BiVec<T> {
    pub fn new() -> Self {
        let mut pos = Vec::with_capacity(10);
        pos.resize_with(10, Default::default);
        let mut neg = Vec::with_capacity(10);
        neg.resize_with(10, Default::default);
         Self {
            pos,
            neg,
        }
    }
    pub fn get(&self, mut idx: i32) -> Option<&T> {
        if idx < 0 {
            idx = -idx - 1;
            return self.neg.get(idx as usize);
        }
        return self.pos.get(idx as usize);
    }
    pub fn set(&mut self, mut idx: i32, val: T) {
        if idx < 0 {
            idx = -idx - 1;
            if idx as usize >= self.neg.capacity() {
                self.neg.resize_with(self.neg.capacity() << 1, Default::default);
            }
            self.neg[idx as usize] = val;
        } else {
            if idx as usize >= self.pos.capacity() {
                self.pos.resize_with(self.pos.capacity() << 1, Default::default);
            }
            self.pos[idx as usize] = val;
        }
    }
}

#[derive(Default)]
pub struct StackNode<T> {
  pub data: T,
  pub next: Option<Rc<StackNode<T>>>
}

impl<T> StackNode<T> {
    pub fn new(data: T, next: Option<Rc<StackNode<T>>>) -> Self {
      Self { data, next }
    }
}

// impl<'a, T> std::ops::Deref for StackNode<'a, T> {
    
// }