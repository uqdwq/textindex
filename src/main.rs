use std::collections::HashMap;
use std::{env, fs};
use std::time::Instant;
use std::str;
mod parser;
mod construct_index;
mod queries;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let filename = &args[2];
    let mut content = fs::read(filename).expect("Something went wrong reading the file");
    content.push(0);
    match mode.as_str() {
        "topk" => top_k(&content, &filename),
        "repeat" => repeat(&content, &filename),
        _ => println!("{} isn't a valid parameter, please use echo, topk or repeat", mode)
    }
}

fn top_k(content: &[u8], filename: &str) {

    // first step is to parse the file, i wont include this step into the time measurement
   

    // i also store the highest k for each patternlength to avoid multiple queues for the same length 
    // useful in for example: example_text_topk_2.txt
    let mut queries: Vec<(i32,i32)> = Vec::new();
    let mut max_query: HashMap<i32, i32> = HashMap::new();
    let mut text_begin: usize = 0;

    // let start_construction = Instant::now();
    parser::parse_content_top_k(&mut queries, &mut max_query, &content, &mut text_begin);
    // starting the timer for construction after parsing, if parsing should be included move it before the parse_content_top_k call
    let start_construction = Instant::now();

    // strings in rust are stored in unicode and know both their length in unicodechars and bytes. so this is cheap 
    let text = &content[text_begin..];

    // 2nd step is to build the textindex i will be using SA (build with prefix doubling ) and LCP-array 


    let mut sa: Vec<i32> = vec![-1; text.len()];
    let mut lcp: Vec<i32> = vec![0; text.len()];
    construct_index::build_sa(text, &mut sa);
    // ultra_naive_suffix_array(&text, &mut sa, &content[text_begin..]);
    // construct_index::build_sa(&text, &mut sa, &content, false);
    construct_index::build_lcp(&text, &sa, &mut lcp);
    
    let duration_construction = start_construction.elapsed();
    // 3rd step queries
    let start_q = Instant::now();

    let result= queries::top_k_query(&queries, &max_query, &sa, &lcp, &content[text_begin..]);
    let duration_q = start_q.elapsed();
    let mut result_string: String = String::from("");
      
    // if the pattern is valid utf8 unicode i will print the so, else i will 
    // just print the byte slice with the interal representation: [b0,b1,b2]
    for i in 0..result.len() {
        let result_bool = result[i].is_ascii();
        if result_bool {
            result_string.push_str(str::from_utf8(result[i]).unwrap());
            result_string.push(';');
        } else {
            let x = format!("{:?}", result[i]);
            result_string.push_str(&x);
            result_string.push(';');
        }
    }
    result_string.pop();
    println!("RESULT algo=topk name=danielmeyer construction_time={:?} query_time={:?} solutions={:?} file={}", duration_construction.as_millis(), duration_q.as_millis(),result_string, filename)
}





fn repeat(text: &[u8], filename: &str) {
    let start_construction = Instant::now();
    // let sat = SuffixTable::new(content);
    let mut sa: Vec<i32> = vec![-1; text.len()];
    let mut lcp: Vec<i32> = vec![0; text.len()];

    construct_index::build_sa(&text, &mut sa);
    construct_index::phi_lcp(&mut lcp, &sa, &text);

    let duration_construction = start_construction.elapsed();
    let start_q = Instant::now();
    let result_val = queries::longest_tandem_repeat(&sa, &lcp);
    // result is starting point and length of a so 2* vor aa
    let start = result_val.0 as usize;
    let end = (result_val.0 + 2 * result_val.1) as usize;
    let duration_q = start_q.elapsed();
    let result_bool = text[start..end].is_ascii();
    let result: String;
    // if the pattern is valid utf8 unicode i will print the so, else i will 
    // just print the byte slice with the interal representation: [b0,b1,b2]
    if result_bool {
        result = match str::from_utf8(&text[start..end]) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    } else {
        result = format!("{:?}", &text[start..end]);
    }
    println!("RESULT algo=repeat name=danielmeyer construction_time={} query_time={} solutions={} file={}", duration_construction.as_millis(), duration_q.as_millis(),result, filename);
}    

