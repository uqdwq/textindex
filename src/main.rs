use std::collections::HashMap;
use std::{env, fs};
use std::time::Instant;

mod parser;
mod construct_index;
mod queries;

mod prefix_double;

mod test;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mode = &args[1];
    let filename = &args[2];
    let mut content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // strings in rust arent NUL-terminated so we need to add a NULBYTE as sentinel
    content.push('\x00');
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
    let mut queries: Vec<(i32,i32)> = Vec::new();
    let mut max_query: HashMap<i32, i32> = HashMap::new();
    let mut text_begin: usize = 0;

    // let start_construction = Instant::now();
    parser::parse_content_top_k(&mut queries, &mut max_query, &content, &mut text_begin);
    // starting the timer for construction after parsing, if parsing should be included move it before the parse_content_top_k call
    let start_construction = Instant::now();

    // strings in rust are stored in unicode and know both their length in unicodechars and bytes. so this is cheap 
    let text = content[text_begin..].as_bytes();

    // 2nd step is to build the textindex i will be using SA (build with SAIS) and LCP-array 


    let mut sa: Vec<i32> = vec![-1; text.len()];
    let mut lcp: Vec<i32> = vec![0; text.len()];
    prefix_double::build_sa(text, &mut sa);
    // ultra_naive_suffix_array(&text, &mut sa, &content[text_begin..]);
    // construct_index::build_sa(&text, &mut sa, &content, false);
    construct_index::build_lcp(&text, &sa, &mut lcp);
    
    let duration_construction = start_construction.elapsed();
    // 3rd step queries
    let start_q = Instant::now();
    //println!("{:?}", sa);
    let result: String = queries::top_k_query(&queries, &max_query, &sa, &lcp, &content[text_begin..]);
    let duration_q = start_q.elapsed();

    println!("RESULT algo=topk name=danielmeyer construction_time={:?} query_time={:?} solutions={} file={}", duration_construction.as_millis(), duration_q.as_millis(),result, filename)
}





fn repeat(content: &str, filename: &str) {
    let start_construction = Instant::now();
    let text = content.as_bytes();
    // let sat = SuffixTable::new(content);
    let mut sa: Vec<i32> = vec![-1; text.len()];
    let mut lcp: Vec<i32> = vec![0; text.len()];

    construct_index::ultra_naive_suffix_array(&text, &mut sa, &content);
    construct_index::phi_lcp(&mut lcp, &sa, &text);

    let duration_construction = start_construction.elapsed();
    let start_q = Instant::now();
    // println!("{:?}", sat.suffix_bytes(1)[2]);
    let result_val = queries::longest_tandem_repeat(&sa, &lcp);
    let start = result_val.0 as usize;
    let end = (result_val.0 + 2 * result_val.1) as usize;
    let result = &content[start..end];
    let duration_q = start_q.elapsed();
    println!("RESULT algo=repeat name=danielmeyer construction_time={} query_time={} solutions={} file={}", duration_construction.as_millis(), duration_q.as_millis(),result, filename)

}

