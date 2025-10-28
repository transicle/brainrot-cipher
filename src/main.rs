use std::{collections::HashMap, env, fs, process};

mod words;

fn build_mapping(contents: &str) -> HashMap<char, String> {
    let mut map = HashMap::new();

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once('=') {
            if let Some(c) = key.chars().next() {
                map.insert(c, value.to_string());
            }
        }
    }

    map
}

fn replace_chars(input: &str, map: &HashMap<char, String>) -> String {
    input
        .chars()
        .filter_map(|c| map.get(&c))
        .cloned()
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: brainrot <text_or_file_path>");
        process::exit(1);
    }

    let input = &args[1];
    let text = if std::path::Path::new(input).exists() {
        fs::read_to_string(input).unwrap_or_else(|_| {
            eprintln!("Failed to read file: {}", input);
            process::exit(1);
        })
    } else {
        input.clone()
    };

    let map = build_mapping(words::WORDS);
    let replaced = replace_chars(&text, &map);

    println!("{}", replaced);
}
