use bit_vec::BitVec;
use std::cmp::{max, min};
use std::env;

fn solve_impl(n: isize, visit: &mut BitVec, row: isize, col: isize) -> usize {
    if row == n && col == n {
        return 1;
    }
    let mut ans = 0;
    let d = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (dr, dc) in d {
        let new_row = row + dr;
        let new_col = col + dc;
        if min(new_row, new_col) < 0 || max(new_row, new_col) > n {
            continue;
        }
        let new_pos = new_row * (n + 1) + new_col;
        if visit.get(new_pos as usize).unwrap() {
            continue;
        }
        visit.set(new_pos as usize, true);
        ans += solve_impl(n, visit, new_row, new_col);
        visit.set(new_pos as usize, false);
    }
    ans
}

fn solve(n: usize) -> usize {
    let mut visit = BitVec::from_elem((n + 1) * (n + 1), false);
    visit.set(0, true);
    solve_impl(n as isize, &mut visit, 0, 0)
}

fn main() {
    let n = env::args().nth(1).unwrap().parse().unwrap();
    println!("oneesan({}*{})={}", n, n, solve(n));
}
