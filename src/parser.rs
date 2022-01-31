use std::collections::HashMap;

pub fn parse_content_top_k(queries: &mut Vec<(i32, i32)>, max_query: &mut HashMap<i32, i32>, content: &[u8], text_begin: &mut usize)  {
    
    let mut abort_at = 0;
    for (i,line) in content.split(|num| *num == b'\n').enumerate() {
        if i == abort_at + 1 && abort_at > 0 {
            break;
        } else if i == 0 {
            *text_begin = line.len() + 1;
            abort_at = match std::str::from_utf8(line) {
                Ok(v) => v.parse::<usize>().unwrap(),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        } else {
            *text_begin += line.len() + 1;
            let line_str = match std::str::from_utf8(line) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let split_line = line_str.split(" ").collect::<Vec<&str>>();
            let query = (split_line[0].parse::<i32>().unwrap(),split_line[1].parse::<i32>().unwrap());
            queries.push(query);
            let max = max_query.entry(query.0).or_insert(0);
            if query.1 > *max {
                *max = query.1;
            } 
        }
    }
    // find first \n to get amount of queries
    // for (i,line) in content.lines().enumerate() { 
    //     if i == abort_at + 1  && abort_at > 0 {
    //         break;
    //     } else if i == 0 {
    //         *text_begin += line.len() + 1;
    //         abort_at = line.parse::<usize>().unwrap();
    //     } else {
    //         *text_begin += line.len() + 1;
    //         let split_line = line.split(" ").collect::<Vec<&str>>();
    //         let query = (split_line[0].parse::<i32>().unwrap(),split_line[1].parse::<i32>().unwrap());
    //         queries.push(query);
    //         let max = max_query.entry(query.0).or_insert(0);
    //         if query.1 > *max {
    //             *max = query.1;
    //         } 
    //     }
    // }
}

#[test]
fn basic_parse() {
    let mut queries: Vec<(i32,i32)> = Vec::new();
    let mut max_query: HashMap<i32, i32> = HashMap::new();
    let mut text_begin: usize = 0;
    let content = String::from("3\n1 2\n1 5\n2 3\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    parse_content_top_k(&mut queries, &mut max_query, &content.as_bytes(), &mut text_begin);
    assert_eq!(queries.len(), 3);
    assert_eq!(&content[text_begin..], "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(text_begin,14);
}