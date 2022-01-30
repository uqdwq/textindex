use std::collections::{HashMap};


pub fn top_k_query(queries: &Vec<(i32, i32)>, max_query: &HashMap<i32, i32>, sa: &Vec<i32>, lcp: &Vec<i32>, text: &str) -> String {

    // we count all duplicated substring for asked lengths

    // we asked for the k-most common pattern of length l
    // 3 cases:
    // 1. no pattern of length l occurrence more than once -> we take the k-smallest pattern of length l <- easy with suffix array
    // 2. less than k pattern of length l occure more than once -> we take (k - o) smallest pattern of length l <- o is |pattern legnth l with occ > 1|
    // 3. at least k patterns occure more than once, we sort them by occ/ length at select the kth
    
    // algo to count occ of duplicated patterns
    // this is inspired by the formula to calculate |unique substrings| in a string: (n^2 + n)/2 - sum(lcp-array)
    // we scan through lcp and sa
    // if lcp[i] > 0 we count up for all substrings t[sa[i]..sa[i] + j] with 1 <= j <= lcp[i]
    // here i will use an hashmap "[String -> (occurrence,first occurrence)]"
    // when we first insert a string into the map we keep track from which sa position it came,
    // this gives an easy way to check for lexiographic order
    // also we make sure to only count up for substring of an asked length
    let mut longest_pattern  = 0;
    for i in max_query.iter() {
        if *i.0 > longest_pattern {
            longest_pattern = *i.0;
        }
    }
    // println!("long {}", longest_pattern);
    let mut lrs = 0;
    for i in 0..lcp.len() {
        if lcp[i] > lrs {
            lrs = lcp[i];
        }
    }
    // println!("{:?}", sa);
    // println!("{:?}", lcp);
    // println!("max pattern {}", longest_pattern);
    // scanning once through lcp and sa
    let mut map_by_l: HashMap<i32, HashMap<&str, (i32, i32)>> = HashMap::new();
    for i in max_query.iter() {
        map_by_l.insert(*i.0 as i32, HashMap::new());
    }
    for i in 0..sa.len() {
        let sa_i = sa[i];
        let lcp_i = lcp[i];

        // we can limited to search to longest asked pattern length
        let min = i32::min(lcp_i, longest_pattern as i32);
        // scan all duplicated substring in suffix sa[i]
        for j in 0..min {
            // println!("{}", j);
            // this might be useful for sparse queries: asked lengths l=1,10000,10000000,10000000000 etc
            if map_by_l.contains_key(&(&j + 1)) {
                // this gives us the hashmap for length j 
                let map_l = map_by_l.entry(j + 1).or_insert(HashMap::new());
                // we insert into map_l with key is a reference to our substring
                // if it is the first time we see this substring we set the first_occ to i 
                // => the sooner we discover it the smaller it is 
                // then we add 1 to the amount of occs.
                let pattern = map_l.entry(&text[sa_i as usize..(sa_i + j + 1) as usize]).or_insert((0, i as i32));
                pattern.0 += 1;
            }
        }

    }
    // println!("{:?}", map_by_l);
    let mut map_k = HashMap::new();
    for i in map_by_l.iter() {
        if *i.0 == 1 {
            println!("{:?}", i.1);
        }
        let mut all_scores: Vec<(i32, i32)> = i.1.into_iter()
                                        .map(|(_id, score)| *score)
                                        .collect();
        all_scores.sort_by(|&(a0, a1), &(b0, b1)| a0.cmp(&b0).then(a1.cmp(&b1).reverse()));
        if *i.0 == 1 {
            println!("{:?}", all_scores);
        }
        let k = map_k.entry(*i.0).or_insert(Vec::new());
        let max = usize::min(*max_query.get(i.0).unwrap() as usize, all_scores.len());
        for _ in 0..max {
            k.push(all_scores.pop().unwrap());
            
        }
        // println!("{} {:?}", *i.0, k);

    }
    let mut result = "".to_owned();
    for (length, k) in queries {
        let vec = map_k.get(length).unwrap();
        // println!("{:?}", vec);
        if *k > vec.len() as i32 {
            // we need the kth and already found vec.len with more than 1
            let mut find = k - vec.len() as i32;
            for i in 0..sa.len() {
                let sa_i = sa[i];
                let lcp_i = lcp[i];
                let suffix_length = sa.len() as i32 - sa_i;
                if suffix_length >= *length && lcp_i < *length {
                    if find > 1 {
                        find -= 1;
                        // println!("{} more we found {}", find, found);
                    } else {
                        let s_l = (sa_i + length) as usize;
                        result.push_str(&text[sa_i as usize..s_l]);
                        println!("{}", &text[sa_i as usize..s_l]);
                        result.push(';');
                        break;
                    }
                }

            }
        } else {
            // println!("were here {:?} {}", vec[*k as usize - 1],length);
            let sa_i = sa[vec[*k as usize - 1].1 as usize];
            let begin = sa_i as usize;
            // println!("{}", &text[begin..]);
            let end = begin + *length as usize;
            result.push_str(&text[begin..end]);
            result.push(';');
        }
    }
    
    // and then collect them into 

    // now we have collected all the occurence of each relvant substring it's time to sort them
    result.pop();
    return result;

}



pub fn longest_tandem_repeat(sa: &Vec<i32>, lcp: &Vec<i32>) -> (i32, i32) {
    // find longest repeating substring <- is upper bound for longest tandem repeat
    let mut lrs = 0;
    for i in 0..lcp.len() {
        if lcp[i] > lrs {
            lrs = lcp[i];
        }
    }
    // we scan sa and lcp once
    let mut longest_tandem = 0;
    let mut tandem_start = 0; 
    let mut share_lcp = 0; 
    let mut current_lcp = 0;
    for i in 0..sa.len() {
        let lcp_i = lcp[i];
        // println!("lcp_i {} curr_lcp {}", lcp_i, current_lcp);
        // if we have to common prefix it cant be a tandem
        if lcp[i] == 0 {
            current_lcp = 0;
            continue;
        }
        // if the common prefix is smaller than the longest tandem we already we dont need to check
        if lcp[i] < longest_tandem {
            current_lcp = lcp[i];
            continue;
        }
        // now we have a lcp[i] > 0 and its a possible candidate for |a| 
        // idea if the lcp[1] = lcp[2] = lcp[3], 0,1,2,3 have the same lcp and we have to check them all
        // println!("share {}", share_lcp);
        for j in 1..(share_lcp+2) {
            
            let begin_t = i32::min(sa[i], sa[i - j]);
            let end_t = i32::max(sa[i], sa[i - j]);
            // println!("sa_i {} b {} e {} {}",sa[i], begin_t, end_t, lcp_i);
            if begin_t + lcp_i == end_t {
                longest_tandem = lcp_i;
                tandem_start = begin_t;
            }
        }

        if lcp_i == current_lcp {
            share_lcp += 1;
        } else {
            current_lcp = lcp_i;
            share_lcp = 1;
        }

        // we found that the lrs is a tandem can stop the search
        if longest_tandem == lrs {
            break;
        }


    }
    return (tandem_start, longest_tandem);
}

//this will produce a longest tandem but wont take order into account

// pub fn walkthrough2() {
//     let text = String::from("banaananaanana\x00");
//     let mut sa = vec![-1; text.len()];
//     let mut lcp: Vec<i32> = vec![0; text.len()];
//     build_sa(text.as_bytes(), &mut sa);
//     build_lcp(&text.as_bytes(), &sa, &mut lcp);
//     println!("lcp  {:?}", lcp);
//     let x = longest_tandem_repeat(&text.as_bytes(), &sa, &lcp);
//     println!("{:?}", x);
//     let y = ultra_naive_tandem(&text);
//     assert!(x.1 == y.1);
// }
// #[test]
// fn problemsheet2() {
//     let mut text = fs::read_to_string("testfiles/example_text_repeats_2.txt").expect("232");
//     let mut sa = vec![-1; text.len()];
//     let mut lcp: Vec<i32> = vec![0; text.len()];
//     build_sa(text.as_bytes(), &mut sa);
//     build_lcp(&text.as_bytes(), &sa, &mut lcp);
//     println!("lcp  {:?}", lcp);
//     let x = longest_tandem_repeat(&text.as_bytes(), &sa, &lcp);
//     println!("{:?}", x);
//     let y = ultra_naive_tandem(&text);
//     assert!(x.1 == y.1);
// }
