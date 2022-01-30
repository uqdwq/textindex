// pub fn build_sa(text: &[u8], sa: &mut Vec<i32>, content: &str, debug: bool) {
    
// }


pub fn build_lcp(text: &[u8], sa: &Vec<i32>, lcp: &mut Vec<i32>) {
    phi_lcp(lcp, sa, text)
}

// used as correctness test
pub fn ultra_naive_suffix_array(text: &[u8], sa: &mut Vec<i32>, content: &str) {
    let mut suffixes: Vec<&str> = Vec::new();
    for offset in 0..text.len() {
        suffixes.push(&content[offset..])
    }
    suffixes.sort();
    for (i, s) in suffixes.iter().enumerate() {
        sa[i] = (text.len() - s.len()) as i32
    }
}

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
        }
        phi[i] = l;
        l = std::cmp::max(0, l - 1);
    }
    for i in 0..sa.len() {
        lcp[i] = phi[sa[i] as usize];
    }
}
