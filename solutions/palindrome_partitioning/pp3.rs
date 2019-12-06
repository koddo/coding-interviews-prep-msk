// https://leetcode.com/problems/palindrome-partitioning-iii

use std::cmp::*;
use std::collections::BinaryHeap;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let v : Vec<u8> = s.chars().map(|x|x as u8).collect();
        let N = v.len();

        // jumps from begin to all ends(inclusive) of palindrome
        let mut Jump = vec![vec![Vec::<usize>::new(); N]; (N / 2 + 1)];
        for i in (0..N).rev() {
            if i > 0 {
                let mut error_n = if v[i - 1] == v[i] {0} else {1};
                Jump[error_n][i - 1].push(i);
                let l_lim = min(i - 1, N - 1 - i);
                for j in 1..(l_lim + 1) {
                    if v[i - 1 - j] != v[i + j] {
                        error_n += 1;
                    }
                    Jump[error_n][i - j - 1].push(i + j);
                }
            }

            let mut error_n = 0;
            Jump[error_n][i].push(i);
            let l_lim = min(i, N - 1 - i);
            for j in 1..(l_lim + 1) {
                if v[i - j] != v[i + j] {
                    error_n += 1;
                }
                Jump[error_n][i - j].push(i + j);
            }
        }
        // each Jump[b] is sorted by e DESC
        let mut shortest_way = vec![vec![N as i32; N]; (N / 2 + 1)];
        shortest_way[0][0] = 0;
        // sort order is -error_n, -dist, +pos
        let mut q = BinaryHeap::<(i32, i32, usize)>::new();
        q.push((-0, -0, 0));
        loop {
            let (er1, d1, b) = q.pop().unwrap();
            let (er0, d) = (-er1, -d1);
            if b >= N {
                return er0;
            }
            if d >= k {
                continue; // jump limit exceeded
            }
            for er in 0..(N / 2 + 1 - er0 as usize) {
                for i in 0..Jump[er][b].len() {
                    let new_pos = Jump[er][b][i] + 1;
                    q.push((-(er0 + er as i32), -(d + 1), new_pos));
                }
            }
        }
    }
}
