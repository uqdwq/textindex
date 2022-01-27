use std::arch::x86_64::_mm_test_all_ones;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Instant;
use suffix::SuffixTable;
use std::{env, fs};
use crate::construct_index::{ultra_naive_suffix_array, build_lcp, build_sa, ultra_naive_lcp, phi_lcp};
pub fn test() {}


#[test]
fn vorlesung_naive() {
    let text = String::from("ababcabcabba\x00");
    let corr = [12, 11, 0, 8, 5, 2, 10, 1,9, 6, 3, 7, 4];
    let mut sa = vec![-1; text.len()];
    let mut lcp = vec![-1; text.len()];
    ultra_naive_suffix_array(text.as_bytes(), &mut sa, &text);
    let mut assert = true;
    for (i,s) in sa.iter().enumerate() {
        if *s != corr[i] {
            assert = false;
        }
    }
    ultra_naive_lcp(&mut lcp, &sa, text.as_bytes());
    phi_lcp(&mut lcp, &sa, text.as_bytes());
    println!("{:?}", lcp);

    assert!(assert)
}

#[test]
fn problemsheet1_naive() {
    let mut text = fs::read_to_string("testfiles/example_text_repeats_1.txt").expect("232");
    text.push('\x00');
    let mut sa = vec![-1; text.len()];
    let mut lcp = vec![-1; text.len()];
    ultra_naive_suffix_array(text.as_bytes(), &mut sa, &text);
    let start = Instant::now();
    let corr = SuffixTable::new(&text);
    let mut assert = true;
    println!("{}", start.elapsed().as_millis());
    for i in 0..sa.len() {
        if sa[i] != corr.table()[i] as i32 {
            assert = false
        }
    }
    println!("{}", start.elapsed().as_millis());
    ultra_naive_lcp(&mut lcp, &sa, text.as_bytes());
    println!("{}", start.elapsed().as_millis());
    let mut lcp2 = vec![-1; text.len()];
    phi_lcp(&mut lcp2, &sa, text.as_bytes());
    println!("{}", start.elapsed().as_millis());
    for i in 0..sa.len() {
        if lcp[i] != lcp2[i] {
            assert = false;
            break;
        }
    }
    println!("{}", start.elapsed().as_millis());
    assert!(assert)
}

#[test]
fn vorlesung() {
    let text = String::from("ababcabcabba\x00");
    let corr = [12, 11, 0, 8, 5, 2, 10, 1,9, 6, 3, 7, 4];
    let mut sa = vec![-1; text.len()];
    build_sa(text.as_bytes(), &mut sa, &text);
    let mut assert = true;
    for (i,s) in sa.iter().enumerate() {
        if *s != corr[i] {
            assert = false;
        }
    }
    println!("{:?}", sa);
    assert!(assert)
}

#[test]
fn camel() {
    let text = String::from("camel\x00");
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
    let text = String::from("abracadabra\x00");
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

#[test]
fn problemsheet1() {
    let mut text = fs::read_to_string("testfiles/example_text_repeats_1.txt").expect("232");
    text.push('\x00');
    let mut sa = vec![-1; text.len()];
    build_sa(text.as_bytes(), &mut sa, &text);
    let corr = SuffixTable::new(&text);
    let mut assert = true;
    for i in 0..sa.len() {
        let x = sa[i] as usize;
        if &text[x..] != corr.suffix(i) {
            assert = false
        }
    }
    assert!(assert)
}

#[test]
fn problemsheet2() {
    let mut text = fs::read_to_string("testfiles/example_text_repeats_2.txt").expect("232");
    text.push('\x00');
    let mut sa = vec![-1; text.len()];
    build_sa(text.as_bytes(), &mut sa, &text);
    let corr = SuffixTable::new(&text);
    let mut assert = true;
    for i in 0..sa.len() {
        let x = sa[i] as usize;
        if &text[x..] != corr.suffix(i) {
            assert = false
        }
    }
    assert!(assert)
}