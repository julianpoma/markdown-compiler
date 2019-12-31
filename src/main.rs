/**
 * https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/
 */
fn usage() {
    let description = env!("CARGO_PKG_DESCRIPTION");
    println!("{}", description);
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("{}", "=".repeat(description.len()));
}

fn main() {
    usage();
}
