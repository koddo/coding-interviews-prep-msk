// https://leetcode.com/problems/palindrome-partitioning/submissions/

fn recur(S : &Vec<u8>, Jump : &Vec<Vec<usize>>, N : usize, e : usize, depth : usize) -> Vec<Vec<String>> {
    if e == 0 {
        vec![Vec::with_capacity(depth);1] // passing depth + with_capacity gives 50% speed boost
    } else {
        let mut res = Vec::<Vec<String>>::new();
        Jump[e - 1].iter().for_each(|&b| {
            let mut subRes = recur(S, Jump, N, b, depth + 1);
            let word : String = std::str::from_utf8(&S[b..e]).unwrap().to_string();
            subRes.iter_mut().for_each(|seq|seq.push(word.clone()));
            res.append(&mut subRes);
        });
        res
    }
}


impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let v : Vec<u8> = s.chars().map(|x|x as u8).collect();
        let N = v.len();

        // jumps from end(inclusive) to the all begins of palindrome
        let mut Jump = vec![Vec::<usize>::new(); N];
        for i in 0..N {
            Jump[i].push(i);
            let l_lim = std::cmp::min(i, N - 1 - i);
            for j in 1..(l_lim + 1) {
                if v[i - j] != v[i + j] {
                    break;
                }
                Jump[i + j].push(i - j);
            }
            if i > 0 && v[i - 1] == v[i] {
                Jump[i].push(i - 1);
                let l_lim = std::cmp::min(i - 1, N - 1 - i);
                for j in 1..(l_lim + 1) {
                    if v[i - 1 - j] != v[i + j] {
                        break;
                    }
                    Jump[i + j].push(i - j - 1);
                }
            }
        }
        recur(&v, &Jump, N, N, 0)
    }
}