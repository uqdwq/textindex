
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Instant;
use suffix::SuffixTable;
use std::{env, fs};

use crate::test;

pub fn build_sa(text: &[u8], sa: &mut Vec<i32>, content: &str) {
    let start_construction = Instant::now();
    // .rev() changes iter direction to from right to left
    let mut last_b: u8 = std::u8::MAX;
    let mut last_t = -1;
    let mut buckets_end = vec![0; 256];
    let mut buckets_begin = vec![0; 256];
    let mut lms: Vec<usize> = Vec::new();
    let mut type_map: HashMap<usize, bool> = HashMap::new();
    println!("first {}", start_construction.elapsed().as_millis());
    for (i, b) in text.iter().enumerate().rev() {
        if b < &last_b {
            last_t = -1;
            type_map.insert(i,false);// S type suffix
        } else if b > &last_b {
            if last_t == -1 {
                lms.push(i+1);
            }
            type_map.insert(i,true);// L type suffix
            last_t = 1;
        }
        last_b = *b;
        buckets_begin[*b as usize] +=1;
        buckets_end[*b as usize] +=1;

    }
    println!("LMS count: {}", lms.len());
    
    println!("lms and bucketsize {}", start_construction.elapsed().as_millis());
    let mut carry: usize = 0;
    for b in buckets_end.iter_mut() {
        *b += carry;
        carry = *b;
    }
    let mut carry: usize = 0;
    for b in buckets_begin.iter_mut() {
        let temp = carry;
        carry += *b as usize;
        *b = temp;
    }

    println!("bucket {}", start_construction.elapsed().as_millis());
    let bucket_copy_end = buckets_end.clone();
    println!("copy {}", start_construction.elapsed().as_millis());
    
    for suffix in lms.iter().rev() {
        let index = buckets_end[text[*suffix as usize] as usize]- 1;
        sa[index] = *suffix as i32;
        buckets_end[text[*suffix as usize] as usize] -= 1;
    }
    println!("place lms {}", start_construction.elapsed().as_millis());
    
    buckets_end = bucket_copy_end;
    for i in 0..sa.len() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_l(&text, ind as usize){
            let bucket_begin = buckets_begin[text[ind as usize] as usize];
            sa[bucket_begin] = ind;
            buckets_begin[text[ind as usize] as usize] += 1;
        }
    }
    println!("forward pass {}", start_construction.elapsed().as_millis());
    for i in (1..sa.len()).rev() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_s(&text, ind as usize){
            let bucket_end = buckets_end[text[ind as usize] as usize];
            sa[bucket_end - 1] = ind;
            buckets_end[text[ind as usize] as usize] -= 1;
        }
    }
    println!("last backwards pass {}", start_construction.elapsed().as_millis());
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

// used as correctness test
pub fn ultra_naive_suffix_array(text: &[u8], sa: &mut Vec<i32>, content: &str) {
    let start_construction = Instant::now();
    let mut suffixes: Vec<&str> = Vec::new();
    for offset in 0..text.len() {
        suffixes.push(&content[offset..])
    }
    suffixes.sort();
    for (i, s) in suffixes.iter().enumerate() {
        sa[i] = (text.len() - s.len()) as i32
    }
    println!("naive time {:?}", start_construction.elapsed().as_millis())
}

pub fn ultra_naive_lcp(lcp: &mut Vec<i32>,sa: &Vec<i32>, content: &[u8]) {
    lcp[0] = 0;
    for i in 1..sa.len() {
        let mut l = 0;
        while content[(sa[i - 1] + l) as usize] == content[(sa[i] + l) as usize] {
            l += 1;
        }
        lcp[i] = l;
    }
}

pub fn phi_lcp(lcp: &mut Vec<i32>,sa: &Vec<i32>, test: &[u8]) {
    let mut phi: Vec<i32> = vec![-1; sa.len()];
    phi[sa.len() -1 ] = sa[sa.len() -1 ];
    for i in 1..sa.len() {
        phi[sa[i] as usize] = sa[i - 1];
    }
    let mut l: i32 = 0;
    println!("check");
    for i in 0..sa.len() {
        let j = phi[i] as usize;
        while test[i + (l as usize)] == test[j + (l as usize)] {
            l = l + 1;
        }
        phi[i] = l;
        l = std::cmp::max(0, l - 1);

    }
    for i in 0..sa.len() {
        lcp[i] = phi[sa[i] as usize];
    }
}