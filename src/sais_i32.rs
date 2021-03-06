
use std::collections::{ HashMap};
pub fn sa_sais_i32(text: &[i32], sa: &mut [i32], rank: i32) {
    // now we can assume text length >= 2
    match text.len() {
        0 => return,
        1 => {
            sa[0] = 0;
            return;
        }
        _ => {}
    }
    println!("Rec with {} and rank {}", sa.len(), rank);
    // println!("{:?}", text);
    // current starting and end position for each bucket, here we have a byte alphabet
    let mut buckets_end: Vec<usize> = vec![0; (rank) as usize];
    let mut buckets_begin: Vec<usize> = vec![0; (rank) as usize];

    // text postion of all LMS suffixes
    let mut lms: Vec<usize> = Vec::new();
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
    let mut copy_e = buckets_end.clone();
    // sort the lms suffixes here we will use a i32 variant of this algorithm recursive
    sort_lms_suffixes(
        &mut lms,
        text,
        sa,
        buckets_end.clone(),
        buckets_begin.clone(),
        &types,
    );
    
    let mut c = 0;
    for i in 0..sa.len() {
        if sa[i] != -1 {
            c += 1;
        }
    }
    println!("c {} lms.len {}", c, lms.len());
    left_pass_sort_l(sa, text, &mut buckets_begin);
    buckets_end = copy_e.clone();
    right_pass_sort_s(sa, text, &mut buckets_end);
    let mut final_counter = 0;
    let mut missing_vec = Vec::new();
    for i in 0..sa.len() {
        if sa[i] == -1 {
            final_counter += 1;
            missing_vec.push(i);
        }
    }
    println!("final counter: {}, {}", final_counter, sa.len());
    let mut  s_t = 0;
    let mut l_t = 0;
    for i in 0..types.len() {
        if types[i] == 0 {
            s_t += 1;
        } else {
            l_t += 1;
        }
    }
    for i in 0..sa.len() {
        if sa[i] != -1 {
            if types[sa[i] as usize] == 0 {
                s_t -= 1;
            } else {
                l_t -= 1;
            }
        }
    }
    let mut collisions_map = HashMap::new();
    for i in 0..sa.len() {
        if sa[i] != -1 {
            let x = collisions_map.entry(sa[i]).or_insert(0);
            *x += 1;
        }
    }
    let mut c = 0;
    for i in collisions_map.iter() {
        if *i.1 > 1 {
            c += 1;
        }
    }
    println!("doubles {}", c);
    println!("missing s {} missing l {} s+t= {} {}", s_t, l_t, s_t + l_t, final_counter)
    //println!("{:?}", missing_vec);
    // println!("final {:?}", sa);
}

pub fn sort_lms_suffixes(
    lms: &mut Vec<usize>,
    text: &[i32],
    sa: &mut [i32],
    mut buckets_end: Vec<usize>,
    mut buckets_begin: Vec<usize>,
    types: &[u8],
) {
    let copy_e = buckets_end.clone();
    let copy_b = buckets_begin.clone();
    for i in 0..sa.len() {
        sa[i] = -1;
    }
    // insert lms suffixes in rev text order into sa
    for lms_s in lms.iter().rev() {
        let index = buckets_end[text[*lms_s] as usize] - 1;
        sa[index] = *lms_s as i32;
        buckets_end[text[*lms_s] as usize] -= 1;
    }

    
    buckets_begin = copy_b.clone();
    // println!("lms {:?}", lms);
 
    left_pass_sort_l(sa, text, &mut buckets_begin);
    // println!("{:?}", sa);
    buckets_end = copy_e.clone();
    right_pass_sort_s(sa, text, &mut buckets_end);
    // println!("{:?}", sa);
    // for i in 0..sa.len() {
    //     if sa[i] > sa.len() as i32 {
    //         println!("Big i {} {}", i , sa.len());
    //     }
    // }
    let mut c = 1;
    for i in 0..sa.len() {
        if sa[i] == -1 {
            c += 1;
        }
    }
    println!("c {}", c);
    let mut num_lms: usize = 0;
    for i in 0..sa.len() {
        let suf_i = sa[i];
        if suf_i == 0 {
            continue;
        }
        if types[suf_i as usize] == 0 && types[(suf_i - 1) as usize] == std::u8::MAX {
            sa[num_lms as usize] = suf_i;
            num_lms += 1;
        }
    }
    for i in num_lms..sa.len() {
        sa[i] = -1;
    }
    // println!("{:?}", sa);
    let mut prev = 0;
    let mut rank = 0;
    for i in 0..num_lms {
        let curr = sa[i];
        if prev == 0 || !lms_substring_eq(&text, &types, curr, prev) {

            rank += 1;
            prev = curr;
        }
        sa[num_lms + (curr/2) as usize] = rank - 1;
    }
    // println!("{:?}", sa);
    let mut end_sa = sa.len() -1;

    for i in (num_lms..(sa.len())).rev() {
        if sa[i] != -1 {
            sa[end_sa] = sa[i];
            end_sa -= 1;
        }
    }


    // println!("{:?}", sa);
    // println!("rank {} num ls {}", rank, num_lms);
    if rank < num_lms as i32 {
        // println!("1");
        let split_at = sa.len() - num_lms;
        let (sa_r, t_r) = sa.split_at_mut(split_at);
        // println!("rsa {:?}", sa_r);
        // println!("rtext {:?}", t_r);
        sa_sais_i32(&t_r, &mut sa_r[..num_lms as usize], rank);
        // println!("len is {}", sa_r[..num_lms as usize].len())
        let mut counter = 0;
        for i in 0..num_lms {
            if sa[i] == -1 {
                counter += 1;
            }
        }
        println!("counter {}", counter)
    } else {
        // println!("2 {:?}", sa);
        for i in 0..num_lms {
            let tmp = sa[(sa.len() - num_lms + i) as usize];
            sa[tmp as usize] = i as i32;
        }
        // println!("{:?}",sa)
    }

    buckets_end = copy_e.clone();
    // println!("rank {} num ls {}", rank, num_lms);
    // println!("r {:?}", sa);
    // println!("LMS {:?}", lms);

    // replace ranknames to suffix index in text
    let mut off = sa.len() - num_lms;
    let mut counter = 0;
    for (i, _) in text.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if types[i as usize] == 0 && types[i as usize - 1 as usize] == std::u8::MAX {
            sa[off] = i as i32;
            off += 1;
            counter += 1;
        }
    }
    println!("We replace {} names", counter);
    let mut counter = 0;
    for i in sa.len()-num_lms..sa.len() {
        if sa[i] == -1 {
            counter += 1;
        }
    }
    println!("after name replacement {} -1", counter);
    // println!("r {:?}", sa);
    let mut counter = 0;
    for i in 0..num_lms {
        if sa[i] == -1 {
            counter += 1;
        }
    }
    println!(" {} -1", counter);
    for i in 0..num_lms {
        let suf_i = sa[i];
        sa[i as usize] = sa[((sa.len() - num_lms) as i32 + suf_i) as usize];
    }
    for i in num_lms..sa.len() {
        sa[i] = -1;
    }
    
    buckets_end = copy_e.clone();
    // println!("hi {}", num_lms);
    let mut count = 0;
    for i in (0..num_lms).rev() {
        if sa[i] == -1 {
            count += 1;
        }
    } 
    println!("count {}", count);
    for i in 0..num_lms {
        if sa[i] == -1 {
            println!("{}", i)
        }
    }
    for i in (0..num_lms).rev() {
        // println!("hi");
        let suf_i = sa[i];
        sa[i] = -1;
        // println!("{} {}",i, sa[i] );
        // println!("{:?}", sa);
        if buckets_end[text[suf_i as usize] as usize] == 0 {
            sa[0] = suf_i as i32;

        } else {
            let index = buckets_end[text[suf_i as usize] as usize] - 1;
            sa[index] = suf_i as i32;
            buckets_end[text[suf_i as usize] as usize] -= 1;
        }

    }
    let mut count = 0;
    for i in 0..sa.len() {
        if sa[i] != -1 {
            count += 1;
        }
    }
    println!("We inserted {} lms suffixes", count);
    // println!("last {:?}", sa);
}

pub fn lms_substring_eq(text: &[i32], types: &[u8], curr: i32, prev: i32) -> bool{ 
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
    text: &[i32],
    types: &mut Vec<u8>,
    lms: &mut Vec<usize>,
    buckets_end: &mut Vec<usize>,
    buckets_begin: &mut Vec<usize>,
    debug: bool,
) {
    //used to find S/L suffixes
    let mut last_b: i32 = std::i32::MAX;
    let mut last_t = -1;

    for (i, b) in text.iter().enumerate().rev() {
        if b > &last_b {
            types[i] = std::u8::MAX;
        }
        last_b = *b;
        buckets_begin[*b as usize] += 1;
        buckets_end[*b as usize] += 1;
    }
    
    let mut c = 0;
    for i in 1..text.len() {
        if types[i] == 0 && types[i - 1] == std::u8::MAX {
            c += 1;
            lms.push(i);
        }
    }
    println!("we init found {}", lms.len());
    println!("But it should be {}", c);
}

pub fn left_pass_sort_l(
    sa: &mut [i32],
    text: &[i32],
    buckets_begin: &mut Vec<usize>,
) {
    for i in 0..sa.len() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_l(text, ind as usize) {
            let bucket_begin = buckets_begin[text[ind as usize] as usize];
            sa[bucket_begin] = ind;

            buckets_begin[text[ind as usize] as usize] += 1;
        }
    }
}

pub fn right_pass_sort_s(
    sa: &mut [i32],
    text: &[i32],
    buckets_end: &mut Vec<usize>,
) {
    for i in (1..sa.len()).rev() {
        let ind = sa[i] - 1;
        if sa[i] != -1 && is_s(text, ind as usize) {
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