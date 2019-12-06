//https://leetcode.com/problems/palindrome-partitioning-ii/

use std::cmp::min;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let v : Vec<u8> = s.chars().map(|x|x as u8).collect();
        let N = v.len();

        // jumps from begin to all ends(inclusive) of palindrome
        let mut Jump = vec![Vec::<usize>::new(); N];
        for i in (0..N).rev() {
            if i > 0 && v[i - 1] == v[i] {
                Jump[i - 1].push(i);
                let l_lim = min(i - 1, N - 1 - i);
                for j in 1..(l_lim + 1) {
                    if v[i - 1 - j] != v[i + j] {
                        break;
                    }
                    Jump[i - j - 1].push(i + j);
                }
            }

            Jump[i].push(i);
            let l_lim = min(i, N - 1 - i);
            for j in 1..(l_lim + 1) {
                if v[i - j] != v[i + j] {
                    break;
                }
                Jump[i - j].push(i + j);
            }
        }
        // each Jump[b] is sorted by e DESC
        let mut shortest_way = vec![N as i32; N];
        for i in 0..Jump[0].len() {
            let e = Jump[0][i] + 1;
            if e >= N {
                return 0;
            }
            shortest_way[e] = 1;
        }
        for L in 1..(N as i32) {
            for b in (1..N).rev() {
                if shortest_way[b] == L {
                    for i in 0..Jump[b].len() {
                        let e = Jump[b][i] + 1;
                        if e >= N {
                            return L;
                        }
                        if shortest_way[e] > L + 1 {
                            shortest_way[e] = L + 1;
                        }
                    }
                }
            }
        }
        0 // unreachable
    }
}
