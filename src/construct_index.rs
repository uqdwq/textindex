use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Instant;
use std::{env, fs};
use suffix::SuffixTable;

use crate::test;
use crate::sais_i32::sa_sais_i32;

pub fn sa_sais(text: &[u8], sa: &mut Vec<i32>, content: &str, debug: bool) {
    // debug/ bench stuff
    let start_construction = Instant::now();
    // current starting and end position for each bucket
    let mut buckets_end: Vec<usize> = vec![0; 256];
    let mut buckets_begin: Vec<usize> = vec![0; 256];
    // text postion of all LMS suffixes
    let mut lms: Vec<usize> = Vec::new();
    // MAP from textpositon i to TYPE of suffix text[i..] false is S-Suffix and true is L-Suffix
    let mut types: Vec<u8> = vec![0; sa.len()];
    print_debug_time("Inits", start_construction.elapsed().as_millis(), debug);
    find_suffix_types(
        text,
        &mut types,
        &mut lms,
        &mut buckets_end,
        &mut buckets_begin,
        debug,
    );
    print_debug_time(
        "Finding Types",
        start_construction.elapsed().as_millis(),
        debug,
    );
    print_debug_types(&types, &lms, debug);
    // calc begin and end for each bucket with given bucketssizes
    let mut carry: usize = 0;
    for b in buckets_end.iter_mut() {
        *b += carry;
        carry = *b;
    }
    let mut carry: usize = 0;
    for b in buckets_begin.iter_mut() {
        let temp = carry;
        carry += *b;
        *b = temp;
    }
    print_debug_time(
        "calculating bucket sizes",
        start_construction.elapsed().as_millis(),
        debug,
    );
    sort_lms_suffixes(
        &mut lms,
        &text,
        sa,
        buckets_end.clone(),
        buckets_begin.clone(),
        &types,
        debug,
    );
    print_debug_time(
        "sorting LMS suffixes",
        start_construction.elapsed().as_millis(),
        debug,
    );
    left_pass_sort_l(sa, text, &mut buckets_begin, debug);
    print_debug_time("L2R Pass", start_construction.elapsed().as_millis(), debug);
    right_pass_sort_s(sa, text, &mut buckets_end, debug);
    print_debug_time("R2L Pass", start_construction.elapsed().as_millis(), debug);
}

pub fn sort_lms_suffixes(
    lms: &mut Vec<usize>,
    text: &[u8],
    sa: &mut Vec<i32>,
    mut buckets_end: Vec<usize>,
    mut buckets_begin: Vec<usize>,
    types: &[u8],
    debug: bool,
) {
    let mut bucket_end_copy = buckets_end.clone();
    let mut bucket_begin_copy = buckets_begin.clone();
    for (i, suffix) in lms.iter().enumerate() {
        println!("{}", suffix);
        if i == lms.len() - 1 {
            break;
        }
        let index = buckets_end[text[*suffix] as usize] - 1;
        sa[index] = *suffix as i32;
        buckets_end[text[*suffix] as usize] -= 1;
    }
    buckets_end = bucket_end_copy;
    println!("{:?}", sa);
    left_pass_sort_l(sa, text, &mut buckets_begin, debug);
    println!("{:?}", sa);
    right_pass_sort_s(sa, text, &mut &mut buckets_end, debug);
    println!("{:?}", sa);
    let mut last_lms_substring = 1;
    for i in 1..sa.len() {
        if sa[i] > 0 {
            if types[sa[i] as usize] == 0 && types[(sa[i] - 1) as usize] == std::u8::MAX {
                sa[last_lms_substring] = sa[i];
                last_lms_substring += 1;
            }
        }
    }
    let mut sar = Vec::new();
    sar.push(1);
    let mut rank = 2;
    let mut mapping = HashMap::new();
    mapping.insert(text.len() - 1, 0);
    for i in 1..last_lms_substring - 1 {
        mapping.insert(sa[i] as usize, i);
        sar.push(rank);
        let mut l = 0;
        let mut no_end_first = true;
        let mut no_end_second = true;
        while no_end_first && no_end_second {
            l += 1;
            if text[(sa[i] + l) as usize] > text[(sa[i + 1] + l) as usize] {
                rank += 1;
                break;
            }
            if types[(sa[i] + l) as usize] != types[(sa[i + 1] + l) as usize] {
                rank += 1;
                break;
            }
            no_end_first = !(types[(sa[i] + l) as usize] == 0
                && types[(sa[i] + l - 1) as usize] == std::u8::MAX);
            no_end_second = !(types[(sa[i + 1] + l) as usize] == 0
                && types[(sa[i + 1] + l - 1) as usize] == std::u8::MAX);
        }
        if no_end_first == true && no_end_second == false {
            rank += 1;
        }
    }
    sar.push(rank);
    mapping.insert(sa[last_lms_substring - 1 as usize] as usize, last_lms_substring - 1);
    let mut t_prime: Vec<i32> = Vec::new();
    for s in lms.iter().rev() {
        let v = mapping.get(s).unwrap();
        t_prime.push(sar[*v])
    }
    if *sar.last().unwrap() as usize == sar.len() {
        println!("restore LMS from {:?}", t_prime)
    } else {
        println!("restore LMS from rec {:?}", t_prime);
        sar = vec![-1; t_prime.len()];
        sa_sais_i32(&t_prime, &mut sar, rank as usize);
        println!("it workd {:?}", sar)
    }
}

pub fn print_debug_types(types: &Vec<u8>, lms: &Vec<usize>, debug: bool) {
    if debug {
        println!("{:?}", types);
        println!("{:?}", lms);
    }
}

pub fn find_suffix_types(
    text: &[u8],
    types: &mut Vec<u8>,
    lms: &mut Vec<usize>,
    buckets_end: &mut Vec<usize>,
    buckets_begin: &mut Vec<usize>,
    debug: bool,
) {
    //used to find S/L suffixes
    let mut last_b: u8 = std::u8::MAX;
    let mut last_t = -1;

    for (i, b) in text.iter().enumerate().rev() {
        if b < &last_b {
            last_t = -1;
        } else if b > &last_b {
            if last_t == -1 {
                lms.push(i + 1);
            }
            types[i] = std::u8::MAX;
            last_t = 1;
        }
        last_b = *b;
        buckets_begin[*b as usize] += 1;
        buckets_end[*b as usize] += 1;
    }
}

pub fn print_debug_time(checkpoint: &str, time: u128, debug: bool) {
    if debug {
        println!("After {}, {}ms passed in SAIS", checkpoint, time)
    }
}
pub fn build_sa(text: &[u8], sa: &mut Vec<i32>, content: &str, debug: bool) {
    sa_sais(text, sa, content, debug)
}

pub fn left_pass_sort_l(
    sa: &mut Vec<i32>,
    text: &[u8],
    buckets_begin: &mut Vec<usize>,
    debug: bool,
) {
    for i in 0..sa.len() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_l(&text, ind as usize) {
            let bucket_begin = buckets_begin[text[ind as usize] as usize];
            sa[bucket_begin] = ind;

            buckets_begin[text[ind as usize] as usize] += 1;
        }
    }
}

pub fn right_pass_sort_s(
    sa: &mut Vec<i32>,
    text: &[u8],
    buckets_end: &mut Vec<usize>,
    debug: bool,
) {
    for i in (1..sa.len()).rev() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_s(&text, ind as usize) {
            let bucket_end = buckets_end[text[ind as usize] as usize];
            if bucket_end == 0 {
                continue;
            }
            println!(
                "For i: {} is SA[{}]: {} and Index: {} is for text[{}] is {} pointing to {}",
                i, i, sa[i], ind, ind, text[ind as usize], bucket_end
            );

            sa[bucket_end - 1] = ind;
            buckets_end[text[ind as usize] as usize] -= 1;
            println!("{:?}", sa);
        }
    }
}

//quick rec hack
pub fn is_l(text: &[u8], i: usize) -> bool {
    if i > text.len() {
        return false;
    }
    if i < text.len() - 2 {
        if text[i] > text[i + 1] {
            return true;
        } else if text[i] < text[i + 1] {
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
        if text[i] > text[i + 1] {
            return false;
        } else if text[i] < text[i + 1] {
            return true;
        } else {
            return is_s(text, i + 1);
        }
    } else {
        return true;
    }
}
pub fn is_lms(text: &[u8], i: usize) -> bool {
    if i > text.len() {
        return false;
    }
    if i < text.len() - 2 {
        if text[i] > text[i + 1] {
            return is_l(text, i - 1);
        } else {
            return false;
        }
    } else {
        return true;
    }
}
pub fn build_lcp(text: &[u8], sa: &Vec<i32>, lcp: &mut Vec<i32>) {
    phi_lcp(lcp, sa, text)
}

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

pub fn ultra_naive_lcp(lcp: &mut Vec<i32>, sa: &Vec<i32>, content: &[u8]) {
    lcp[0] = 0;
    for i in 1..sa.len() {
        let mut l = 0;
        while content[(sa[i - 1] + l) as usize] == content[(sa[i] + l) as usize] {
            l += 1;
        }
        lcp[i] = l;
    }
}

pub fn phi_lcp(lcp: &mut Vec<i32>, sa: &Vec<i32>, text: &[u8]) {
    let mut phi: Vec<i32> = vec![-1; sa.len()];
    phi[sa.len() - 1] = sa[sa.len() - 1];
    for i in 1..sa.len() {
        phi[sa[i] as usize] = sa[i - 1];
    }
    let mut l: i32 = 0;
    println!("check");
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
