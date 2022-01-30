use std::{cmp::Ordering};




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



// #[test]
// pub fn walkthrough2() {
//     let text = String::from("banaananaanana\x00");
//     let mut sa = vec![-1; text.len()];
//     build_sa(text.as_bytes(), &mut sa);
//     println!("{:?}", sa);
// }
// #[test]
// fn problemsheet2() {
//     let mut text = fs::read_to_string("testfiles/example_text_repeats_2.txt").expect("232");
//     text.push('\x00');
//     let mut sa = vec![-1; text.len()];
//     build_sa(text.as_bytes(), &mut sa);
//     assert!(true)
// }