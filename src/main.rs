use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
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

fn parse_file_name(f: &str) -> (&str, String) {
    let parts: Vec<&str> = f.split('.').collect();

    match parts.len() {
        1 => {
            let mut file_name = String::from(parts[0]);
            file_name.push_str(".md");
            (parts[0], file_name)
        }
        2 => (parts[0], String::from(f)),
        _ => panic!("[ERROR] Invalid file name"),
    }
}

fn open_file(file_name: &str) -> File {
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

    file
}

fn write_output(tokens: &Vec<String>, name: &str) {
    let mut output_file = String::from(name);
    output_file.push_str(".html");

    let mut outfile = File::create(output_file).expect("[ERROR] Could not create file!");

    for line in tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ERROR] Could not write to output file");
        outfile
            .write_all("\n".as_bytes())
            .expect("[ERROR] Could not write to output file");
    }

    println!("[INFO] Done!");
}

fn parse_markdown(f: &str) {
    println!("{}", get_title());

    let (name, file_name) = parse_file_name(f);

    println!("[INFO] Parsing {} ...", file_name);

    let file = open_file(&file_name);

    let mut _h1_tag: bool = false;
    let mut _h2_tag: bool = false;
    let mut _p_tag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let file_buffer = BufReader::new(file);

    for line in file_buffer.lines() {
        let line_content: String = line.unwrap().to_string();
        let splitted_line: Vec<&str> = line_content.split(' ').collect();

        match splitted_line.first() {
            Some(&"#") => {
                _h1_tag = true;
                _h2_tag = false;
                _p_tag = false;
            },
            Some (&"##") => {
                _h1_tag = false;
                _h2_tag = true;
                _p_tag = false;
            }
            _ => {
                _h1_tag = false;
                _h2_tag = false;
                _p_tag = true;
            }
        }

        let mut output = String::new();

        if _h1_tag {
            output.push_str("<h1>");
            output.push_str(&line_content[2..]);
            output.push_str("</h1>");
        }

        if _h2_tag {
            output.push_str("<h2>");
            output.push_str(&line_content[3..]);
            output.push_str("</h2>");
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

    write_output(&tokens, name);
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
