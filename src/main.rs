/**
 * https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/
 */
fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(", ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn print_title() {
    println!("{}", get_title());
}

fn print_help() {
    println!("{}", String::from("Usage: mkdc <file.md>"));
}

fn print_separation_bars(separator: &str, longitude: usize) {
    println!("{}", separator.repeat(longitude));
}

fn parse_markdown(file: &str) {
    print_title();
    println!("[INFO] Parsing {} ...", file);
}

fn usage() {
    let title = get_title();
    println!("{}", title);
    print_help();
    print_separation_bars("=", title.len())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parse_markdown(&args[1]),
        _ => {
            println!("[ ERROR ] You forgot to specify the markdown file to parse!");
            usage();
        },
    }
}
