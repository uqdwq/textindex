use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;


#[derive(Copy, Clone, Eq, PartialEq)]
struct Tuple {
    occ: i32,
    first: i32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Tuple {
    fn cmp(&self, other: &Tuple) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.occ.cmp(&other.occ).then(self.first.cmp(&other.first).reverse())
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Tuple) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn top_k_query<'a>(queries: &Vec<(i32, i32)>, max_query: &HashMap<i32, i32>, sa: &Vec<i32>, lcp: &Vec<i32>, text: &'a[u8]) -> Vec<&'a[u8]> {

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
    let mut lrs = 0;
    for i in 0..lcp.len() {
        if lcp[i] > lrs {
            lrs = lcp[i];
        }
    }
    // scanning once through lcp and sa
    let mut map_by_l: HashMap<i32, HashMap<&[u8], Tuple>> = HashMap::new();
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
            // this might be useful for sparse queries: asked lengths l=1,10000,10000000,10000000000 etc
            if map_by_l.contains_key(&(&j + 1)) {
                // this gives us the hashmap for length j 
                let map_l = map_by_l.entry(j + 1).or_insert(HashMap::new());
                // we insert into map_l with key as reference to our substring
                // if it is the first time we see this substring we set the first_occ to i 
                // => the sooner we discover it the smaller it is 
                // then we add 1 to the amount of occs.
                let pattern = map_l.entry(&text[sa_i as usize..(sa_i + j + 1) as usize]).or_insert(Tuple {occ: 0, first: i as i32});
                pattern.occ += 1;
            }
        }

    }
    // now we are using a prioqueue to get all need patterns for each length
    let mut map_k = HashMap::new();
    for i in map_by_l.iter() {
        let mut bin: BinaryHeap<Tuple> = i.1.into_iter()
                                        .map(|(_id, score)| *score)
                                        .collect();
        // all_scores.sort_by(|&(a0, a1), &(b0, b1)| a0.cmp(&b0).then(a1.cmp(&b1).reverse()));
        let k = map_k.entry(*i.0).or_insert(Vec::new());
        let max = usize::min(*max_query.get(i.0).unwrap() as usize, bin.len());
        for _ in 0..max {
            k.push(bin.pop().unwrap());
            
        }


    }
    let mut result = Vec::new();
    // Now were anwsering each request
    for (length, k) in queries {
        let vec = map_k.get(length).unwrap();
        // case 1: the pattern isnt in the vec and instead is a unique pattern so we have to find it in den suffix array
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
                    } else {
                        let s_l = (sa_i + length) as usize;
                        result.push(&text[sa_i as usize..s_l]);
                        break;
                    }
                }

            }
        // case 2 it is are duplicated pattern and it is a easy look up
        } else {
            let sa_i = sa[vec[*k as usize - 1].first as usize];
            let begin = sa_i as usize;
            let end = begin + *length as usize;
            result.push(&text[begin..end]);
        }
    }
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
    // idea for each suffix in sa we will check if the there is a p=aa with |a| = lcp[i + 1]
    // example lcp[i + 1] is 2
    // check now if sa[i] and sa[i+1] "allign"
    // if the smaller sa value + lcp_i is the bigger sa value 
    // we can do this until we find a lcp val thats smaller than lcp[i + 1] 
    for i in 1..(sa.len()-1) {
        
        let lcp_i = lcp[i + 1];
        
        // were looking for pattern with |a| = lcp_i so we wont find any with this suffix
        if lcp_i == 0 {
            continue;
        }
        // we already found a bigger one
        if lcp_i < longest_tandem {
            continue;
        }
        // compare with everyone higher up in sa
        for j in (i+1)..(sa.len()) {
            
            // until we find a smaller lcp val 
            // all values can fit the allignment
            if lcp_i > lcp[j] {
                break;
            }
            // order for allginment
            let begin_t = i32::min(sa[i], sa[j]);
            let end_t = i32::max(sa[i], sa[j]);
            // allginment
            // if our to val have at least lcp_i many lcps
            //  suffix in sa[i]: abra$
            //  suffix in sa[j]: abraabra$  <- lcp_i is 4 
            //  and we check if     abra$
            //                  abraabra$ fits
            if begin_t + lcp_i == end_t {
                longest_tandem = lcp_i;
                tandem_start = begin_t;
                break;
            }
        }
        // there isnt anything better 
        if longest_tandem == lrs {
            break;
        }


    }
    return (tandem_start, longest_tandem);
}
