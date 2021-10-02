use async_recursion::async_recursion;
use async_std::sync::Arc;
use bit_vec::BitVec;
use futures::executor::{block_on, ThreadPool};
use futures::prelude::*;
use futures::stream::FuturesUnordered;
use futures::task::SpawnExt;
use std::cmp::{max, min};
use std::time::Instant;

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

#[async_recursion]
async fn solve_async(
    n: isize,
    mut visit: BitVec,
    row: isize,
    col: isize,
    parallel_depth: usize,
    pool: Arc<ThreadPool>,
) -> usize {
    if row == n && col == n {
        return 1;
    }
    if parallel_depth == 0 {
        return solve_impl(n, &mut visit, row, col);
    }
    let mut ans = 0;
    let d = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut cf = FuturesUnordered::new();
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
        {
            let mut visit = visit.clone();
            visit.set(new_pos as usize, true);
            let pool_new = pool.clone();
            cf.push(
                pool.spawn_with_handle(async move {
                    solve_async(n, visit, new_row, new_col, parallel_depth - 1, pool_new).await
                })
                .unwrap(),
            );
        }
    }
    while let Some(res) = cf.next().await {
        ans += res;
    }
    ans
}

fn solve(n: usize) -> usize {
    let mut visit = BitVec::from_elem((n + 1) * (n + 1), false);
    visit.set(0, true);
    let pool = Arc::new(ThreadPool::new().expect("Failed to build pool"));
    let parallel_depth = n * 2; // enough tasks with small overhead
    block_on(solve_async(n as isize, visit, 0, 0, parallel_depth, pool))
}

fn main() {
    for n in 1.. {
        let start = Instant::now();
        let ans = solve(n);
        println!(
            "oneesan({}) = {}, elapsed: {:.3}s",
            n,
            ans,
            start.elapsed().as_secs_f64()
        );
    }
}
