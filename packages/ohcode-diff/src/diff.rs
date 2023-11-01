pub mod diff {
    use std::rc::Rc;

    use crate::structure::*;

    pub fn compute(seq1: &Vec<usize>, seq2: &Vec<usize>) -> Vec<DiffRes> {
        if seq1.len() == 0 || seq2.len() == 0 {
            return vec![DiffRes::new(
                RowRange::new(0, seq1.len()),
                RowRange::new(0, seq2.len()),
            )];
        }
        let get_x_after_snake = |x: i32, y: i32| -> i32 {
            if x < 0 || y < 0 {
              return x;
            }
            let mut x = x as usize;
            let mut y = y as usize;
            while x < seq1.len() && y < seq2.len() && seq1[x] == seq2[y] {
                x += 1;
                y += 1;
            }
            x as i32
        };
        let mut bi_v = BiVec::new();
        bi_v.set(0, get_x_after_snake(0, 0));
        let mut paths = BiVec::new();

        paths.set(
            0,
            match bi_v.get(0) {
                Some(0) | None => None,
                Some(&x) => Some(Rc::new(StackNode::new((0, 0, x), None))),
            },
        );
        let mut k = 0;
        let mut d: i32 = 0;
        'LOOP: loop {
            d += 1;
            let lb = -(d.min(seq2.len() as i32 + (d & 1)));
            let ub = d.min(seq1.len() as i32 + (d & 1));
            k = lb;
            while k <= ub {
                let max_x_of_d_line_top = if k == ub {
                    -1
                } else {
                    *bi_v.get(k + 1).unwrap() as i32
                };

                let max_x_of_d_line_left = if k == lb {
                    -1
                } else {
                    *bi_v.get(k - 1).unwrap() as i32 + 1
                };

                let x = (max_x_of_d_line_top.max(max_x_of_d_line_left)).min(seq1.len() as i32);
                let y = x as i32 - k;
                if x > seq1.len() as i32 || y > seq2.len() as i32 {
                    k += 2;
                    continue;
                }
                let new_max_x = get_x_after_snake(x, y);
                bi_v.set(k, new_max_x);

                let last_path = if x as i32 == max_x_of_d_line_top {
                    paths.get(k + 1).unwrap()
                } else {
                    paths.get(k - 1).unwrap()
                };
                let last_path = match last_path {
                    Some(x) => Some(Rc::clone(x)),
                    None => None,
                };
                paths.set(
                    k,
                    if x == new_max_x {
                        last_path
                    } else {
                        Some(Rc::new(StackNode::new((x, y, new_max_x - x), match last_path {
                            Some(x) => Some(Rc::clone(&x)),
                            _ => None
                        } )))
                    },
                );

                if *bi_v.get(k).unwrap() == seq1.len() as i32
                    && seq1.len() as i32 == seq2.len() as i32 + k
                {
                    break 'LOOP;
                }
                k += 2;
            }
        };
        let mut path = match paths.get(k).unwrap() {
            Some(x) => Some(Rc::clone(x)),
            _ => None,
        };
        let mut res = vec![];
        let mut lasr_aligning_pos_s1 = seq1.len();
        let mut lasr_aligning_pos_s2 = seq2.len();
        loop {
            let (end_x, end_y) = match &path {
                Some(t) => ((t.data.0 + t.data.2) as usize, (t.data.1 + t.data.2) as usize),
                _ => (0, 0),
            };
            if end_x != lasr_aligning_pos_s1 || end_y != lasr_aligning_pos_s2 {
                res.push(DiffRes::new(
                    RowRange::new(end_x, lasr_aligning_pos_s1),
                    RowRange::new(end_y, lasr_aligning_pos_s2),
                ))
            }
            if let Some(t) = path.take() {
                lasr_aligning_pos_s1 = t.data.0 as usize;
                lasr_aligning_pos_s2 = t.data.1 as usize;
                path = match &t.next {
                    Some(x) => Some(Rc::clone(x)),
                    _ => None
                } 
            } else {
                break;
            }
        }
        res.reverse();
        return res;
    }

    pub fn optimize_line_diff(
        seq1: &Vec<usize>,
        seq2: &Vec<usize>,
        line_diff_res: Vec<DiffRes>,
    ) -> Vec<DiffRes> {
        if line_diff_res.len() == 0 {
            return line_diff_res;
        };
        let mut res = vec![];
        line_diff_res
            .into_iter()
            .enumerate()
            .for_each(|(idx, mut cur)| {
                if idx == 0 {
                    return res.push(cur);
                }
                let pre = res.last_mut().unwrap();
                if cur.origin.is_empty() || cur.modified.is_empty() {
                    let len = cur.origin.start_row - pre.origin.end_row;
                    let mut d = 1;
                    for _ in 1..=len {
                        if seq1.get(cur.origin.start_row - d).unwrap_or(&usize::MAX)
                            != seq1.get(cur.origin.end_row - d).unwrap_or(&usize::MAX)
                            || seq2.get(cur.modified.start_row - d).unwrap_or(&usize::MAX)
                                != seq2.get(cur.modified.end_row - d).unwrap_or(&usize::MAX)
                        {
                            break;
                        }
                        d += 1;
                    }
                    d -= 1;
                    if d == len {
                        pre.origin.end_row = cur.origin.end_row - len;
                        pre.modified.end_row = cur.modified.end_row - len;
                    }
                    cur.delta(d);
                }
                res.push(cur);
            });

        for idx in 0..res.len() - 1 {
            let next = &res[idx + 1];
            let cur = &res[idx];
            if cur.origin.is_empty() || cur.modified.is_empty() {
                let len = next.origin.start_row - cur.origin.end_row;
                let mut d = 0;
                for _ in 0..len {
                    if seq1.get(cur.origin.start_row + d).unwrap_or(&usize::MAX)
                        != seq1.get(cur.origin.end_row + d).unwrap_or(&usize::MAX)
                        || seq2.get(cur.modified.start_row + d).unwrap_or(&usize::MAX)
                            != seq2.get(cur.modified.end_row + d).unwrap_or(&usize::MAX)
                    {
                        break;
                    }
                    d += 1;
                }
                if d == len {
                    res[idx + 1].origin.end_row = res[idx].origin.end_row - len;
                    res[idx + 1].modified.end_row = res[idx].modified.end_row - len;
                    continue;
                }
                if d > 0 {
                    res[idx].delta(d);
                }
            }
        }
        return res;
    }

    // pub fn remove_random_matches(
    //     seq1: Vec<usize>,
    //     seq2: Vec<usize>,
    //     line_diff_res: Vec<DiffRes>,
    // ) -> Vec<DiffRes> {
    //   if line_diff_res.len() == 0 {
    //     return line_diff_res;
    //   }
    //   let mut diff = line_diff_res;
    //   let mut cnt = 0;
    //   let mut should_repeat;
    //   loop {
    //     should_repeat = false;
    //     let mut result = vec![];
    //     diff.into_iter().enumerate().for_each(|(idx, cur)| {
    //       if idx == 0 {
    //         return result.push(cur);
    //       }
    //       let last_res = result.last().unwrap();
    //       let should_join_diff = |before: DiffRes, after: DiffRes|-> bool {
    //         let unchanged_range = RowRange::new(last_res.origin.end_row, cur.origin.start_row);

    //         return false;
    //       };
    //     });
    //     diff = result;
    //   }

    //   return vec![];
    // }
}
