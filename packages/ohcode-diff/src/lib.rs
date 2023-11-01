extern crate wasm_bindgen;

use diff::diff::{compute, optimize_line_diff};

use wasm_bindgen::prelude::*;

mod structure;
mod diff;


// #[wasm_bindgen]
// pub fn compute_diff(original_lines: String, modified_lines: String) -> Box<[u32]> {
//   let original_lines: Vec<&str> = original_lines.split('\n').collect();
//   let modified_lines: Vec<&str> = modified_lines.split('\n').collect();
//   if original_lines.len() == 1 && original_lines[0].len() == 0 || modified_lines.len() == 1 && modified_lines[0].len() == 0 {
//     return vec![].into();
//   }

//   let mut hash: HashMap<&str, usize> = HashMap::new();
//   let get_hash = |str: &str| -> usize { **hash.get(str.trim()).get_or_insert(&hash.len()) };

//   let src_lines: Vec<usize> = original_lines.iter().map(|str| get_hash(str)).collect();
//   let tgt_lines: Vec<usize> = modified_lines.iter().map(|str| get_hash(str)).collect();

//   let mut line_diff_res = compute(src_lines, tgt_lines);
  

//   let mut res = vec![];
//   return res.into();
// }


#[wasm_bindgen]
pub fn compute_diff(original_lines: Box<[u32]>, modified_lines: Box<[u32]>) -> Box<[u32]> {

  let src_lines: Vec<usize> = original_lines.into_iter().map(|&x| x as usize).collect();
  let tgt_lines: Vec<usize> = modified_lines.into_iter().map(|&x| x as usize).collect();

  let line_diff_res = compute(&src_lines, &tgt_lines);

  let line_diff_res = optimize_line_diff(&src_lines, &tgt_lines, line_diff_res);

  let mut res = vec![];
  for x in line_diff_res {
    res.push(x.origin.start_row as u32);
    res.push(x.origin.end_row as u32);
    res.push(x.modified.start_row as u32);
    res.push(x.modified.end_row as u32);
  }
  return res.into();
}

#[test]
fn test() {
  println!("{:?}", compute_diff(Box::new([1,2,3,4]), Box::new([2,4,3,5])));
}