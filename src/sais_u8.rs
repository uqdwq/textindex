use std::collections::VecDeque;
use crate::sais_i32::sa_sais_i32;
pub fn sa_sais_u8(text: &[u8], sa: &mut Vec<i32>, debug: bool) {
    // now we can assume text length >= 2
    match text.len() {
        0 => return,
        1 => {
            sa[0] = 0;
            return;
        }
        _ => {}
    }

    // current starting and end position for each bucket, here we have a byte alphabet
    let mut buckets_end: Vec<usize> = vec![0; 256];
    let mut buckets_begin: Vec<usize> = vec![0; 256];

    // text postion of all LMS suffixes
    let mut lms: VecDeque<usize> = VecDeque::new();
    // MAP from textpositon i to TYPE of suffix text[i..] false is S-Suffix and true is L-Suffix
    let mut types: Vec<u8> = vec![0; text.len()];

    // i know we can avoid a lot of this extra used space but i was just trying to make it work first
    find_suffix_types(
        text,
        &mut types,
        &mut lms,
        &mut buckets_end,
        &mut buckets_begin,
        true,
    );
    // types know contains if a suffix is S/L
    // lms contains all LMS suffixes in text order

    // calc begin and end for each bucket with given buckets sizes
    let mut carry: usize = 0;
    for b in buckets_end.iter_mut() {
        *b += carry;
        carry = *b;
    }
    carry = 0;
    for b in buckets_begin.iter_mut() {
        let temp = carry;
        carry += *b;
        *b = temp;
    }

    // sort the lms suffixes here we will use a i32 variant of this algorithm recursive
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
    println!("finale {:?}", sa)
}

pub fn sort_lms_suffixes(
    lms: &mut VecDeque<usize>,
    text: &[u8],
    sa: &mut Vec<i32>,
    mut buckets_end: Vec<usize>,
    mut buckets_begin: Vec<usize>,
    types: &[u8],
) {
    let mut copy_e = buckets_end.clone();
    let mut copy_b = buckets_begin.clone();
    
    // insert lms suffixes in rev text order into sa
    for lms_s in lms.iter().rev() {
        let index = buckets_end[text[*lms_s] as usize] - 1;
        sa[index] = *lms_s as i32;
        buckets_end[text[*lms_s] as usize] -= 1;
    }
    buckets_end = copy_e.clone();

    left_pass_sort_l(sa, text, &mut buckets_begin);
    //println!("{:?}", sa);
    right_pass_sort_s(sa, text, &mut &mut buckets_end);
    //println!("{:?}", sa);

    let mut num_lms: usize = 0;
    for i in 0..sa.len() {
        let suf_i = sa[i];
        // suf 0 is by definition no LMS so we dont need to check
        if suf_i == 0 {
            continue;
        }
        if types[suf_i as usize] == 0 && types[suf_i as usize- 1 as usize] == std::u8::MAX {
            sa[num_lms as usize] = suf_i;
            num_lms += 1;
        }
    }
    for i in num_lms..sa.len() {
        sa[i] = -1;
    }
    //println!("{:?}", sa);
    let mut prev = 0;
    let mut rank = 0;
    println!("{}", num_lms);
    for i in 0..num_lms {
        let curr = sa[i];
        if prev == 0 || !lms_substring_eq(&text, &types, curr, prev) {

            rank += 1;
            prev = curr;
        }
        sa[num_lms + (curr/2) as usize] = rank - 1;
    }
    //println!("{:?}", sa);
    let mut end_sa = sa.len() -1;
    for i in (num_lms..(sa.len())).rev() {
        if sa[i] != -1 {
            sa[end_sa] = sa[i];
            end_sa -= 1;
        }
    }
    //println!("{:?}", sa);
    if rank < num_lms as i32 {
        let split_at = sa.len() - num_lms;
        let (sa_r, t_r) = sa.split_at_mut(split_at);
        //println!("rsa {:?}", sa_r);
        //println!("rtext {:?}", t_r);
        // println!("tr {:?}", t_r);
        sa_sais_i32(&t_r, &mut sa_r[..num_lms as usize], rank);
        let mut counter = 0;
        for i in 0..num_lms {
            if sa[i] == -1 {
                counter += 1;
            }
        }
        println!("counter {}", counter)
    } else {
        for i in 0..num_lms {
            let tmp = sa[(sa.len() - num_lms + i) as usize];
            sa[tmp as usize] = i as i32;
        }
    }
    // println!("rank {} num ls {}", rank, num_lms);
    // println!("r {:?}", sa);
    // println!("LMS {:?}", lms);
    let mut off = sa.len() - num_lms;
    for (i, _) in text.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if types[i as usize] == 0 && types[i as usize - 1 as usize] == std::u8::MAX {
            sa[off] = i as i32;
            off += 1;
        }
    }
    // println!("r {:?}", sa);

    for i in 0..num_lms {
        let suf_i = sa[i];
        sa[i as usize] = sa[((sa.len() - num_lms) as i32 + suf_i) as usize];
    }
    for i in num_lms..sa.len() {
        sa[i] = -1;
    }
    
    buckets_end = copy_e.clone();
    let mut count = 0;
    for i in (0..num_lms).rev() {
        if sa[i] == -1 {
            count += 1;
        }
    } 
    println!("count {}", count);
    // println!("hi {}", num_lms);
    for i in (0..num_lms).rev() {
        // println!("hi");
        let suf_i = sa[i];
        sa[i] = -1;
        println!("{} {}",i, suf_i );
        // println!("{:?}", sa);
        if buckets_end[text[suf_i as usize] as usize] == 0 {
            sa[0] = suf_i as i32;
        } else {
            let index = buckets_end[text[suf_i as usize] as usize] - 1;
            sa[index] = suf_i as i32;
            buckets_end[text[suf_i as usize] as usize] -= 1;
        }

    }
}

pub fn lms_substring_eq(text: &[u8], types: &[u8], curr: i32, prev: i32) -> bool{ 
    // println!("{:?} vs {:?}", &text[curr as usize..], &text[prev as usize..]);
    // sentiel is unique lms substring so we can just skip it and make our lifes
    // below a lot easier
    if prev  as usize == text.len()- 1 {
        return false;
    }
    let mut l = 0;
    let mut none_ended = true;
    while none_ended {
        // println!("{} {} {}", prev, text[(curr + l) as usize], text[(prev + l)as usize]);
        l += 1;
        // substring ends on next LMS suffix 
        let prev_ended = types[(prev + l) as usize] == 0 && types[(prev + l - 1) as usize] == std::u8::MAX;
        let curr_ended = types[(curr + l) as usize] == 0 && types[(curr + l - 1) as usize] == std::u8::MAX;
        if prev_ended != curr_ended {
            return  false;
        }
        if text[(curr + l) as usize] != text[(prev + l) as usize] {
            return false;
        }
        if types[(curr + l) as usize] != types[(prev + l) as usize] {
            return false;
        }
        none_ended = !prev_ended && !curr_ended;

    }
    return true;
}

pub fn find_suffix_types(
    text: &[u8],
    types: &mut Vec<u8>,
    lms: &mut VecDeque<usize>,
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
                lms.push_front(i + 1);
            }
            types[i] = std::u8::MAX;
            last_t = 1;
        }
        last_b = *b;
        buckets_begin[*b as usize] += 1;
        buckets_end[*b as usize] += 1;
    }
}

pub fn left_pass_sort_l(
    sa: &mut Vec<i32>,
    text: &[u8],
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
    text: &[u8],
    buckets_end: &mut Vec<usize>,
) {
    for i in (1..sa.len()).rev() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_s(&text, ind as usize) {
            let bucket_end = buckets_end[text[ind as usize] as usize];
            if bucket_end == 0 {
                continue;
            }
            // println!(
            //     "For i: {} is SA[{}]: {} and Index: {} is for text[{}] is {} pointing to {}",
            //     i, i, sa[i], ind, ind, text[ind as usize], bucket_end
            // );

            sa[bucket_end - 1] = ind;
            buckets_end[text[ind as usize] as usize] -= 1;
            // println!("{:?}", sa);
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
