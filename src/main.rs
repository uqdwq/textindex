use std::collections::HashMap;
use std::{env, fs};
use std::time::Instant;

mod parser;
mod construct_index;
mod queries;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mode = &args[1];
    let filename = &args[2];
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    match mode.as_str() {
        "topk" => top_k(&content, &filename),
        "repeat" => repeat(&content, &filename),
        "echo" => print!("{}",content),
        _ => println!("{} isn't a valid parameter, please use echo, topk or repeat", mode)
    }
}

fn top_k(content: &str, filename: &str) {

    // first step is to parse the file, i wont include this step into the time measurement
   

    // i also store the highest k for each patternlength to avoid multiple queues for the same length 
    // useful in for example: example_text_topk_2.txt
    let mut queries: Vec<(u32,u32)> = Vec::new();
    let mut max_query: HashMap<u32, u32> = HashMap::new();
    let mut text_begin: usize = 0;

    // let start_construction = Instant::now();
    parser::parse_content_top_k(&mut queries, &mut max_query, &content, &mut text_begin);

    // starting the timer for construction after parsing, if parsing should be included move it before the parse_content_top_k call
    let start_construction = Instant::now();

    // strings in rust are stored in unicode and know both their length in unicodechars and bytes. so this is cheap 
    let text = content[text_begin..].as_bytes();

    // 2nd step is to build the textindex i will be using SA (build with SAIS) and LCP-array 

    // strings in rust arent NUL-terminated so we need to add a NULBYTE as sentinel later
    let mut sa: Vec<i32> = vec![-1; text.len() + 1];
    let mut lcp: Vec<i32> = vec![0; text.len() + 1];
    construct_index::build_sa(&text, &mut sa);
    construct_index::build_lcp(&text, &sa, &mut lcp);

    let duration_construction = start_construction.elapsed();
    // 3rd step queries
    let start_q = Instant::now();
    let mut result: String = queries::top_k_query(&queries, &max_query, &sa, &lcp);
    let duration_q = start_q.elapsed();

    println!("RESULT algo=topk name=danielmeyer construction_time={:?} query_time={:?} solutions={} file={}", duration_construction.as_millis(), duration_q.as_millis(),result, filename)
}





fn repeat(content: &str, filename: &str) {
    let start_construction = Instant::now();
    let text = content.as_bytes();
    let mut sa: Vec<i32> = vec![-1; text.len() + 1];
    let mut lcp: Vec<i32> = vec![0; text.len() + 1];
    construct_index::build_sa(&text, &mut sa);
    construct_index::build_lcp(&text, &sa, &mut lcp);
    let duration_construction = start_construction.elapsed();
    let start_q = Instant::now();
    let result = queries::longest_tandem_repeat(&text, &sa, &lcp);
    let duration_q = start_q.elapsed();
    println!("RESULT algo=repeat name=danielmeyer construction_time={} query_time={} solutions={} file={}", duration_construction.as_millis(), duration_q.as_millis(),result, filename)

}

