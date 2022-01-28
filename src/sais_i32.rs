use std::{time::Instant, collections::HashMap};



pub fn sa_sais_i32(text: &[i32], sa: &mut Vec<i32>, rank: usize) {
    // debug/ bench stuff
    let debug = true;
    // current starting and end position for each bucket
    let mut buckets_end: Vec<usize> = vec![0; rank + 1];
    let mut buckets_begin: Vec<usize> = vec![0; rank + 1];
    // text postion of all LMS suffixes
    let mut lms: Vec<usize> = Vec::new();
    // MAP from textpositon i to TYPE of suffix text[i..] false is S-Suffix and true is L-Suffix
    let mut types: Vec<u8> = vec![0; text.len()];
    find_suffix_types(
        text,
        &mut types,
        &mut lms,
        &mut buckets_end,
        &mut buckets_begin,
    );
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
    sort_lms_suffixes(
        &mut lms,
        &text,
        sa,
        buckets_end.clone(),
        buckets_begin.clone(),
        &types,
    );
    left_pass_sort_l(sa, text, &mut buckets_begin);
    right_pass_sort_s(sa, text, &mut buckets_end);
}

pub fn find_suffix_types(
    text: &[i32],
    types: &mut Vec<u8>,
    lms: &mut Vec<usize>,
    buckets_end: &mut Vec<usize>,
    buckets_begin: &mut Vec<usize>,
) {
    //used to find S/L suffixes
    let mut last_b: i32 = std::i32::MAX;
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

pub fn sort_lms_suffixes(
    lms: &mut Vec<usize>,
    text: &[i32],
    sa: &mut Vec<i32>,
    mut buckets_end: Vec<usize>,
    mut buckets_begin: Vec<usize>,
    types: &[u8],
) {
    println!("i32 sort XD {:?}", lms);
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
    left_pass_sort_l(sa, text, &mut buckets_begin);
    println!("{:?}", sa);
    right_pass_sort_s(sa, text, &mut &mut buckets_end);
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
    let mut t_prime = Vec::new();
    for s in lms.iter().rev() {
        let v = mapping.get(s).unwrap();
        t_prime.push(sar[*v])
    }
    if *sar.last().unwrap() == sar.len() {
        println!("restore LMS from {:?}", t_prime)
    } else {
        println!("restore LMS from rec {:?}", t_prime)
    }
}

pub fn left_pass_sort_l(
    sa: &mut Vec<i32>,
    text: &[i32],
    buckets_begin: &mut Vec<usize>,
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
    text: &[i32],
    buckets_end: &mut Vec<usize>,
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

pub fn is_l(text: &[i32], i: usize) -> bool {
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

pub fn is_s(text: &[i32], i: usize) -> bool {
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