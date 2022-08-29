use clap::Parser;
use std::collections::HashMap;


#[derive(Parser)]
struct Cli {
    pattern: String,
}

fn main(){
    let args: Cli = Cli::parse();
    println!("{}", args.pattern);
    let mut map = HashMap::new();
    for (_, char) in args.pattern.chars().enumerate() {
        *map.entry(char).or_insert(0) += 1
    }
    let mut sorted_chars: Vec<char> = map.keys().cloned().collect();
    sorted_chars.sort();
    for char in sorted_chars {
        let count = map.get(&char).unwrap();
        println!("{} {}", char, count);
    }
}
