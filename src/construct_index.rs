use std::collections::{BTreeMap, BTreeSet};

pub fn build_sa(text: &[u8], sa: &mut Vec<i32>, content: &str) {

    // .rev() changes iter direction to from right to left
    let mut last_b: u8 = std::u8::MAX;
    let mut last_t = -1;
    let mut buckets: BTreeMap<u8, usize> = BTreeMap::new();
    let mut lms: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, b) in text.iter().enumerate().rev() {
        if b < &last_b {
            last_t = -1
        } else if b > &last_b {
            if last_t == -1 {
                lms.insert(&content[i+1..],i+1);
                println!("{:?}", &content[i+1..])
            }
            last_t = 1;
        }
        last_b = *b;
        let bucket = buckets.entry(*b).or_insert(0);
        *bucket += 1;

    }
    println!("{:?}", &buckets);
    println!("{:?}", &lms);
    let mut carry: usize = 0;
    for b in buckets.iter_mut() {
        *b.1 += carry;
        carry = *b.1;
    }
    println!("{:?}", &buckets);
    for suffix in lms.iter().rev() {
        println!("{:?}", suffix);
        let index = buckets.get(&suffix.0.as_bytes()[0]).unwrap() - 1;
        sa[index] = *suffix.1 as i32;
        let bucket = buckets.entry(suffix.0.as_bytes()[0]).or_insert(0);
        *bucket -= 1;
    } 
    println!("{:?}", &sa);
}

pub fn build_lcp(text: &[u8], sa: &Vec<i32>, lcp: &Vec<i32>) {}



#[test]
fn basic_sa() {
    let text = String::from("ababcabcabba$");
    let mut sa = vec![-1; text.len()];
    build_sa(text.as_bytes(), &mut sa, &text);
    println!();
    assert!(false)
}