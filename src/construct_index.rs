use std::{cmp::Ordering}; 

pub fn build_lcp(text: &[u8], sa: &Vec<i32>, lcp: &mut Vec<i32>) {
    phi_lcp(lcp, sa, text)
}

pub fn phi_lcp(lcp: &mut Vec<i32>, sa: &Vec<i32>, text: &[u8]) {
    let mut phi: Vec<i32> = vec![-1; sa.len()];
    phi[sa.len() - 1] = sa[sa.len() - 1];
    for i in 1..sa.len() {
        phi[sa[i] as usize] = sa[i - 1];
    }
    let mut l: i32 = 0;
    for i in 0..sa.len() {
        let j = phi[i] as usize;
        while text[i + (l as usize)] == text[j + (l as usize)] {
            l = l + 1;
            if i + l as usize >= text.len() || j + l as usize >= text.len() {
                break;
            } 
        }
        phi[i] = l;
        l = std::cmp::max(0, l - 1);
    }
    for i in 0..sa.len() {
        lcp[i] = phi[sa[i] as usize];
    }
}





#[derive(Debug,PartialEq, Eq)]
struct Suffix {
    index: usize,
    rank: (i32, i32),
}

impl Ord for Suffix {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.0.cmp(&other.rank.0).then_with(|| self.rank.1.cmp(&other.rank.1))
    }
}

impl PartialOrd for Suffix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn build_sa(text: &[u8], sa: &mut Vec<i32>) {
    let mut suffixes: Vec<Suffix> = Vec::new();

    for i in 0..text.len() {
        let rank2: i32;
        if  i < text.len() - 1 {
            rank2 = text[i + 1] as i32;
        } else {
            rank2 = -1;
        }
        suffixes.push(Suffix {index : i, rank :(text[i] as i32, rank2)});
    }

    suffixes.sort();
    let mut ind = vec![0; text.len()];
    
    let mut k = 4;
    while k < 2 * text.len() {
 
        let mut rank = 0;
        let mut prev_rank = suffixes[0].rank.0;
        suffixes[0].rank.0 = rank;
        ind[suffixes[0].index] = 0;

        for i in 1..text.len() {
            if suffixes[i].rank.0 == prev_rank && suffixes[i].rank.1 == suffixes[i - 1].rank.1 {
                prev_rank = suffixes[i].rank.0;
                suffixes[i].rank.0 = rank;
            } else {
                prev_rank = suffixes[i].rank.0;
                rank += 1;
                suffixes[i].rank.0 = rank;
            }
            ind[suffixes[i].index] = i;
        }

        for i in 0..text.len() {
            let next_index = suffixes[i].index + k/2;
            if next_index < text.len() {
                suffixes[i].rank.1 = suffixes[ind[next_index]].rank.0;
            } else {
                suffixes[i].rank.1 = -1;
            }
        }

        suffixes.sort();
        k = 2*k;
        // println!("{:?}", suffixes)
    }

    for i in 0..text.len() {
        sa[i] = suffixes[i].index as i32;
    }

}


// used as correctness test
// pub fn ultra_naive_suffix_array(text: &[u8], sa: &mut Vec<i32>, content: &str) {
//     let mut suffixes: Vec<&str> = Vec::new();
//     for offset in 0..text.len() {
//         suffixes.push(&content[offset..])
//     }
//     suffixes.sort();
//     for (i, s) in suffixes.iter().enumerate() {
//         sa[i] = (text.len() - s.len()) as i32
//     }
// }

// pub fn ultra_naive_lcp(lcp: &mut Vec<i32>, sa: &Vec<i32>, content: &[u8]) {
//     lcp[0] = 0;
//     for i in 1..sa.len() {
//         let mut l = 0;
//         while content[(sa[i - 1] + l) as usize] == content[(sa[i] + l) as usize] {
//             l += 1;
//         }
//         lcp[i] = l;
//     }
// }


