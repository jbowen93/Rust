use std::fs::File;
use std::io::BufReader;
use std::env;

fn sum_file(path: &str) {
    let mut sum: i64 = 0;
    let file = File::open(path);
}
fn main() {
    for argument in env::args() {
        if argument != "./file_sum" {
            println!("{}", argument);
        }
    }
}
