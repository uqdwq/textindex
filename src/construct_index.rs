use std::collections::{BTreeMap, BTreeSet};

pub fn build_sa(text: &[u8], sa: &mut Vec<i32>, content: &str) {

    // .rev() changes iter direction to from right to left
    let mut last_b: u8 = std::u8::MAX;
    let mut last_t = -1;
    let mut buckets_end: BTreeMap<u8, usize> = BTreeMap::new();
    let mut buckets_begin: BTreeMap<u8, usize> = BTreeMap::new();
    let mut lms: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, b) in text.iter().enumerate().rev() {
        if b < &last_b {
            last_t = -1
        } else if b > &last_b {
            if last_t == -1 {
                lms.insert(&content[i+1..],i+1);
            }
            last_t = 1;
        }
        last_b = *b;
        let bucket = buckets_end.entry(*b).or_insert(0);
        let bucket2 = buckets_begin.entry(*b).or_insert(0);
        *bucket += 1;
        *bucket2 += 1;

    }
    let mut carry: usize = 0;
    for b in buckets_end.iter_mut() {
        *b.1 += carry;
        carry = *b.1;
    }
    let mut carry: usize = 0;
    for b in buckets_begin.iter_mut() {
        let temp = carry;
        carry += *b.1;
        *b.1 = temp;
    }
    let bucket_copy = buckets_end.clone();
    for suffix in lms.iter().rev() {
        let index = buckets_end.get(&suffix.0.as_bytes()[0]).unwrap() - 1;
        sa[index] = *suffix.1 as i32;
        let bucket = buckets_end.entry(suffix.0.as_bytes()[0]).or_insert(0);
        *bucket -= 1;
    }
    buckets_end = bucket_copy;
    for i in 0..sa.len() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_l(&text, ind as usize){
            let bucket_begin = buckets_begin.entry(text[ind as usize]).or_insert(0);
            sa[*bucket_begin] = ind;
            *bucket_begin += 1;
        }
    }
    for i in (1..sa.len()).rev() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_s(&text, ind as usize){
            let bucket_end = buckets_end.entry(text[ind as usize]).or_insert(0);
            sa[*bucket_end - 1] = ind;
            *bucket_end -= 1;
        }
    }
}

//quick rec hack
pub fn is_l(text: &[u8], i: usize) -> bool {
    if i > text.len() {
        return false;
    }
    if i < text.len() - 2 {
        if text[i] > text[i+1] {
            return true;
        } else if text[i] < text[i+1] {
            return false;
        } else {
            return is_l(text, i + 1);  
        }
    } else {
        return true;
    }
}

pub fn is_s(text: &[u8], i: usize) -> bool {
    if i > text.len() {
        return false;
    }
    if i < text.len() - 2 {
        if text[i] > text[i+1] {
            return false;
        } else if text[i] < text[i+1] {
            return true;
        } else {
            return is_s(text, i + 1);  
        }
    } else {
        return true;
    }
}
pub fn build_lcp(text: &[u8], sa: &Vec<i32>, lcp: &Vec<i32>) {}



#[test]
fn vorlesung() {
    let text = String::from("ababcabcabba$");
    let corr = [12, 11, 0, 8, 5, 2, 10, 1,9, 6, 3, 7, 4];
    let mut sa = vec![-1; text.len()];
    build_sa(text.as_bytes(), &mut sa, &text);
    let mut assert = true;
    for (i,s) in sa.iter().enumerate() {
        if *s != corr[i] {
            assert = false;
        }
    }
    assert!(assert)
}

#[test]
fn camel() {
    let text = String::from("camel$");
    let mut sa = vec![-1; text.len()];
    build_sa(text.as_bytes(), &mut sa, &text);
    let corr = [5,1,0,3,4,2];
    let mut assert = true;
    for (i,s) in sa.iter().enumerate() {
        if *s != corr[i] {
            assert = false;
        }
    }
    assert!(assert)
}
#[test]
fn abracadabra() {
    let text = String::from("abracadabra$");
    let mut sa = vec![-1; text.len()];
    build_sa(text.as_bytes(), &mut sa, &text);
    let corr = [11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
    let mut assert = true;
    for (i,s) in sa.iter().enumerate() {
        if *s != corr[i] {
            assert = false;
        }
    }
    assert!(assert)
}