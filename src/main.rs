use std::collections::HashMap;
use std::ops::Range;
use std::{env, fs};
use std::time::Instant;
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
    let start_construction = Instant::now();
    let mut x: usize = 0;
    
    let mut queries: Vec<(u32,u32)> = Vec::new();
    let mut text_begin: usize = 0;

    // i also store the highest k for each patternlength to avoid multiple queues for the same length 
    // useful in for example: example_text_topk_2.txt
    let mut max_query: HashMap<u32, u32> = HashMap::new();
    
    parse_content_top_k(&mut queries, &mut max_query, &content, &mut text_begin);

    // strings in rust are stored in unicode and know both their length in unicodechars and bytes. so this is cheap 
    let text = content[text_begin..].as_bytes();


    // 2nd step is to build the textindex i will be using SA (build with SAIS) and LCP-array 

    // strings in rust arent NUL-terminated so we need to add a NULBYTE as sentinel later
    let mut sa: Vec<i32> = vec![-1; text.len() + 1];
    let mut lcp: Vec<i32> = vec![0; text.len() + 1];
    build_sa(&text, &mut sa);
    build_lcp(&text, &sa, &mut lcp);

    let duration_construction = start_construction.elapsed();
    // 3rd step queries
    let start_q = Instant::now();
    let mut result: String = top_k_query(&queries, &max_query, &sa, &lcp);
    let duration_q = start_q.elapsed();

    println!("RESULT algo=topk name=danielmeyer construction_time={:?} query_time={:?} solutions={} file={}", duration_construction.as_millis(), duration_q.as_millis(),result, filename)
}

fn parse_content_top_k(queries: &mut Vec<(u32, u32)>, max_query: &mut HashMap<u32, u32>, content: &str, text: &mut usize)  {}
fn build_sa(text: &[u8], sa: &mut [i32]) {}
fn build_lcp(text: &[u8], sa: &Vec<i32>, lcp: &Vec<i32>) {}
fn top_k_query(queries: &Vec<(u32, u32)>, max_query: &HashMap<u32, u32>, sa: &Vec<i32>, lcp: &Vec<i32>) -> String {
    String::from("Penis;Dick")
}

fn repeat(content: &str, filename: &str) {
    let start_construction = Instant::now();
    let text = content.as_bytes();
    let mut sa: Vec<i32> = vec![-1; text.len() + 1];
    let mut lcp: Vec<i32> = vec![0; text.len() + 1];
    build_sa(&text, &mut sa);
    build_lcp(&text, &sa, &mut lcp);
    let duration_construction = start_construction.elapsed();
    let start_q = Instant::now();
    let result = longest_tandem_repeat(&text, &sa, &lcp);
    let duration_q = start_q.elapsed();
    println!("RESULT algo=repeat name=danielmeyer construction_time={} query_time={} solutions={} file={}", duration_construction.as_millis(), duration_q.as_millis(),result, filename)

}

fn longest_tandem_repeat(text: &[u8], sa: &Vec<i32>, lcp: &Vec<i32>) -> String {
    String::from("PenisPenis")
}