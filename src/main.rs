use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(", ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn print_devider(separator: &str, longitude: usize) {
    println!("{}", separator.repeat(longitude));
}

fn parse_markdown(file_name: &str) {
    println!("{}", get_title());
    println!("[INFO] Parsing {} ...", file_name);

    let file_path = Path::new(file_name);

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => {
            println!(
                "[ERROR] Something went wrong reading the file: {}",
                file_name
            );
            println!("[INFO] Maybe the path is wrong");
            panic!("{}", err.description());
        }
    };

    let mut _h1_tag: bool = false;
    let mut _p_tag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let file_buffer = BufReader::new(file);

    for line in file_buffer.lines() {
        let line_content = line.unwrap().to_string();
        let mut first_char: Vec<char> = line_content.chars().take(1).collect();

        match first_char.pop() {
            Some('#') => {
                _h1_tag = true;
                _p_tag = false;
            }
            _ => {
                _h1_tag = false;
                _p_tag = true;
            }
        }

        let mut output = String::new();

        if _h1_tag {
            output.push_str("<h1>");
            output.push_str(&line_content[2..]);
            output.push_str("</h1>");
        }

        if _p_tag {
            output.push_str("<p>");
            output.push_str(&line_content[..]);
            output.push_str("</p>");
        }

        if output != "<p></p>" {
            tokens.push(output);
        }
    }

    println!("{:?}", tokens);
}

fn usage() {
    let title = get_title();
    println!("{}", title);
    println!("{}", String::from("Usage: mkdc <file.md>"));
    print_devider("=", title.len())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown(&args[1]),
        _ => {
            println!("[ERROR] You forgot to specify the markdown file to parse!");
            usage();
        }
    }
}
