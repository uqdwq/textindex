use std::{env, fs};
use std::time::Instant;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mode = &args[1];
    let filename = &args[2];
    let text = fs::read_to_string(filename).expect("Something went wrong reading the file");
    match mode.as_str() {
        "topk" => top_k(&text),
        "repeat" => repeat(&text),
        "echo" => print!("{}",text),
        _ => println!("{} isn't a valid parameter, please use echo, topk or repeat", mode)
    }
}

fn top_k(text: &str) {
    let mut x: usize = 0;
    let now = Instant::now();
    let queries: Vec<Vec<u32>> = Vec::new();

    for (i, c) in text.chars().enumerate() {
        if c == '\n' {
            x = i;
            break;
        }
    }
    let n = text[0..x].parse::<usize>().unwrap();
    println!("{}", n);
    // omega expensive TODO
    

    // println!("k: {}", n);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed)

}

fn repeat(text: &str) {}